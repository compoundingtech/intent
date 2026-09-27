use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs;
use std::ops::Range;
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SourceLocation {
    pub path: String,
    pub start: SourcePosition,
    pub end: SourcePosition,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GraphReference {
    /// The mechanical syntax that produced this reference.
    pub syntax: &'static str,
    pub source: SourceLocation,
    pub written_target: String,
    pub resolution: &'static str,
    pub target_locations: Vec<SourceLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

pub(super) struct GraphReferenceBuild {
    pub references: Vec<GraphReference>,
    pub contents: BTreeMap<PathBuf, String>,
}

#[derive(Debug)]
struct Document {
    path: PathBuf,
    repo_path: String,
    content: String,
    line_starts: Vec<usize>,
}

impl Document {
    fn read(repository_root: &Path, path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let mut line_starts = vec![0];
        line_starts.extend(
            content
                .bytes()
                .enumerate()
                .filter_map(|(index, byte)| (byte == b'\n').then_some(index + 1)),
        );
        Ok(Self {
            path: path.to_path_buf(),
            repo_path: relative_display(repository_root, path),
            content,
            line_starts,
        })
    }

    fn location(&self, range: Range<usize>) -> SourceLocation {
        SourceLocation {
            path: self.repo_path.clone(),
            start: self.position(range.start),
            end: self.position(range.end),
        }
    }

    fn position(&self, offset: usize) -> SourcePosition {
        let offset = offset.min(self.content.len());
        let line_index = match self.line_starts.binary_search(&offset) {
            Ok(index) => index,
            Err(index) => index.saturating_sub(1),
        };
        let line_start = self.line_starts[line_index];
        SourcePosition {
            line: line_index + 1,
            column: self.content[line_start..offset].chars().count() + 1,
        }
    }
}

#[derive(Clone, Debug)]
struct Definition {
    normalized_id: String,
    artifact: String,
    location: SourceLocation,
}

#[derive(Clone, Debug)]
struct Companion {
    normalized_ordinal: String,
    owner: PathBuf,
    path: PathBuf,
    repo_path: String,
    location: SourceLocation,
}

#[derive(Clone, Debug)]
struct MarkdownLink {
    label: String,
    written_target: String,
    range: Range<usize>,
}

#[derive(Clone, Debug)]
struct ResolvedLink {
    reference: GraphReference,
    target_path: Option<PathBuf>,
}

#[derive(Clone, Debug)]
struct IdOccurrence {
    written: String,
    normalized: String,
    range: Range<usize>,
}

#[derive(Clone, Debug)]
pub(super) struct StructuredId {
    pub id: String,
    pub title: String,
    pub refs: Vec<String>,
    pub refines: Vec<String>,
    pub evidence: String,
    range: Range<usize>,
}

#[derive(Clone, Debug)]
struct ReqTrace {
    written: String,
    range: Range<usize>,
    target_range: Range<usize>,
}

#[derive(Clone, Debug)]
struct OrdinalOccurrence {
    written: String,
    normalized: String,
    range: Range<usize>,
}

pub(super) fn build(
    corpus_root: &Path,
    markdown_files: &[PathBuf],
) -> Result<GraphReferenceBuild, Box<dyn std::error::Error>> {
    let repository_root = repository_root(corpus_root);
    let documents = markdown_files
        .iter()
        .map(|path| Document::read(&repository_root, path))
        .collect::<Result<Vec<_>, _>>()?;

    let definitions = documents
        .iter()
        .flat_map(definitions_in)
        .collect::<Vec<_>>();
    let definitions_by_id = group_definitions(&definitions);
    let companions = documents
        .iter()
        .filter_map(|document| companion_for(&repository_root, document))
        .collect::<Vec<_>>();
    let companions_by_ordinal = group_companions(&companions);
    let companion_by_path = companions
        .iter()
        .map(|companion| (companion.path.clone(), companion.clone()))
        .collect::<BTreeMap<_, _>>();
    let documents_by_path = documents
        .iter()
        .map(|document| (document.path.clone(), document))
        .collect::<BTreeMap<_, _>>();

    let mut references = BTreeSet::new();
    for document in &documents {
        let links = markdown_links_in(document);
        let resolved_links = links
            .iter()
            .map(|link| {
                resolve_markdown_link(
                    corpus_root,
                    &repository_root,
                    document,
                    link,
                    &documents_by_path,
                )
            })
            .collect::<Vec<_>>();
        references.extend(
            resolved_links
                .iter()
                .map(|resolved| resolved.reference.clone()),
        );

        let req_traces = req_traces_in(document);
        for trace in &req_traces {
            references.insert(resolve_req_trace(
                corpus_root,
                &repository_root,
                document,
                trace,
                &resolved_links,
                &definitions_by_id,
            ));
        }

        let definition_ranges = definition_ranges_in(document);
        let req_target_ranges = req_traces
            .iter()
            .map(|trace| trace.target_range.clone())
            .collect::<Vec<_>>();
        for occurrence in id_citations_in(document) {
            if overlaps_any(&occurrence.range, &definition_ranges)
                || overlaps_any(&occurrence.range, &req_target_ranges)
            {
                continue;
            }
            references.insert(resolve_id_citation(
                document,
                occurrence,
                &resolved_links,
                &definitions_by_id,
            ));
        }

        let link_ranges = links
            .iter()
            .map(|link| link.range.clone())
            .collect::<Vec<_>>();
        for occurrence in ordinal_citations_in(document, &companions_by_ordinal) {
            if overlaps_any(&occurrence.range, &link_ranges) {
                continue;
            }
            references.insert(resolve_ordinal_citation(
                corpus_root,
                &repository_root,
                document,
                occurrence,
                &resolved_links,
                &companions_by_ordinal,
                &companion_by_path,
            ));
        }
    }

    Ok(GraphReferenceBuild {
        references: references.into_iter().collect(),
        contents: documents
            .into_iter()
            .map(|document| (document.path, document.content))
            .collect(),
    })
}

fn repository_root(corpus_root: &Path) -> PathBuf {
    for ancestor in corpus_root.ancestors() {
        if ancestor.join(".git").exists() {
            return ancestor.to_path_buf();
        }
    }
    if let Some(context) = corpus_root
        .ancestors()
        .find(|ancestor| ancestor.file_name() == Some(OsStr::new("context")))
    {
        if let Some(parent) = context.parent() {
            return parent.to_path_buf();
        }
    }
    corpus_root.to_path_buf()
}

fn definitions_in(document: &Document) -> Vec<Definition> {
    definition_occurrences(document)
        .into_iter()
        .map(|occurrence| Definition {
            normalized_id: occurrence.normalized,
            artifact: document.repo_path.clone(),
            location: document.location(occurrence.range),
        })
        .collect()
}

fn definition_ranges_in(document: &Document) -> Vec<Range<usize>> {
    definition_occurrences(document)
        .into_iter()
        .map(|occurrence| occurrence.range)
        .collect()
}

fn definition_occurrences(document: &Document) -> Vec<IdOccurrence> {
    prose_lines(&document.content)
        .into_iter()
        .filter_map(|(line_start, line)| {
            let trimmed = line.trim_start();
            let indentation = line.len() - trimmed.len();
            let after_marker = if let Some(rest) = trimmed.strip_prefix("- ") {
                rest
            } else if let Some(rest) = trimmed.strip_prefix("* ") {
                rest
            } else {
                let digits = trimmed.bytes().take_while(u8::is_ascii_digit).count();
                trimmed
                    .get(digits..)
                    .and_then(|rest| rest.strip_prefix(". "))?
            };
            let marker_offset = trimmed.len() - after_marker.len();
            after_marker.strip_prefix("**")?;
            structured_id_at(
                &document.content,
                line_start + indentation + marker_offset,
            )
            .map(|definition| IdOccurrence {
                written: definition.id.clone(),
                normalized: definition.id.to_ascii_uppercase(),
                range: definition.range,
            })
        })
        .collect()
}

pub(super) fn structured_ids_outside_code(content: &str) -> Vec<StructuredId> {
    prose_lines(content)
        .into_iter()
        .filter_map(|(line_start, line)| {
            line.find("**")
                .and_then(|start| structured_id_at(content, line_start + start))
        })
        .collect()
}

fn structured_id_at(content: &str, bold_start: usize) -> Option<StructuredId> {
    let label_start = bold_start.checked_add(2)?;
    let label_tail = content.get(label_start..)?;
    let label_end = label_start + label_tail.find("**")?;
    let label_source = content.get(label_start..label_end)?;
    if label_source.lines().skip(1).any(|line| {
        let trimmed = line.trim_start();
        line.trim().is_empty() || trimmed.starts_with("```") || trimmed.starts_with("~~~")
    }) {
        return None;
    }

    let label = label_source
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let label = label.trim().trim_end_matches(':').trim();
    let written_id = label
        .split(|character: char| character.is_whitespace() || character == ':')
        .next()?;
    let id = written_id.trim_end_matches('.');
    if !super::looks_like_intent_id(id) {
        return None;
    }
    let id_offset = label_source.find(written_id)?;
    let range = label_start + id_offset..label_start + id_offset + id.len();
    let title = label
        .get(written_id.len()..)
        .unwrap_or_default()
        .trim()
        .to_string();
    let rest_start = label_end + 2;
    let rest_end = content[rest_start..]
        .find('\n')
        .map_or(content.len(), |offset| rest_start + offset);
    let rest = content.get(rest_start..rest_end)?.trim();
    let evidence_start = content[..bold_start]
        .rfind('\n')
        .map_or(0, |offset| offset + 1);
    let evidence = content
        .get(evidence_start..rest_end)?
        .trim()
        .to_string();

    Some(StructuredId {
        id: id.to_string(),
        title: if title.is_empty() {
            id.to_string()
        } else {
            title
        },
        refs: super::refs_in_text(rest),
        refines: super::refines_in_text(rest),
        evidence,
        range,
    })
}

fn group_definitions(definitions: &[Definition]) -> BTreeMap<String, Vec<Definition>> {
    let mut grouped = BTreeMap::<String, Vec<Definition>>::new();
    for definition in definitions {
        grouped
            .entry(definition.normalized_id.clone())
            .or_default()
            .push(definition.clone());
    }
    grouped
}

fn companion_for(repository_root: &Path, document: &Document) -> Option<Companion> {
    let parent = document.path.parent()?;
    let directory = parent.file_name()?.to_str()?;
    let file_name = document.path.file_name()?.to_str()?;
    let (normalized_ordinal, owner) = if directory == ".decisions" {
        let digits = leading_ascii_digits(file_name);
        (
            normalize_decision_ordinal(digits?)?,
            parent.parent()?.to_path_buf(),
        )
    } else if directory == ".delta" || directory == "09-delta" {
        (
            normalize_delta_ordinal(file_name.strip_prefix("DELTA-")?.split('-').next()?)?,
            parent.parent()?.to_path_buf(),
        )
    } else {
        return None;
    };

    Some(Companion {
        normalized_ordinal,
        owner,
        path: document.path.clone(),
        repo_path: relative_display(repository_root, &document.path),
        location: document.location(0..first_line_end(&document.content)),
    })
}

fn leading_ascii_digits(value: &str) -> Option<&str> {
    let length = value.bytes().take_while(u8::is_ascii_digit).count();
    (length > 0).then(|| &value[..length])
}

fn normalize_decision_ordinal(value: &str) -> Option<String> {
    value
        .parse::<u64>()
        .ok()
        .map(|number| format!("{number:04}"))
}

fn normalize_delta_ordinal(value: &str) -> Option<String> {
    value
        .parse::<u64>()
        .ok()
        .map(|number| format!("DELTA-{number:03}"))
}

fn group_companions(companions: &[Companion]) -> BTreeMap<String, Vec<Companion>> {
    let mut grouped = BTreeMap::<String, Vec<Companion>>::new();
    for companion in companions {
        grouped
            .entry(companion.normalized_ordinal.clone())
            .or_default()
            .push(companion.clone());
    }
    grouped
}

fn markdown_links_in(document: &Document) -> Vec<MarkdownLink> {
    let mut links = Vec::new();
    for (line_start, line) in prose_lines(&document.content) {
        let bytes = line.as_bytes();
        let mut index = 0;
        while index < bytes.len() {
            let Some(open) = line[index..].find('[').map(|offset| index + offset) else {
                break;
            };
            if (open > 0 && matches!(bytes[open - 1], b'!' | b'\\'))
                || bytes.get(open + 1) == Some(&b'[')
            {
                index = open + 1;
                continue;
            }
            let Some(close) = line[open + 1..].find(']').map(|offset| open + 1 + offset) else {
                break;
            };
            if bytes.get(close + 1) != Some(&b'(') {
                index = close + 1;
                continue;
            }
            let target_start = close + 2;
            let Some(paren_end) = line[target_start..]
                .find(')')
                .map(|offset| target_start + offset)
            else {
                break;
            };
            let raw = line[target_start..paren_end].trim();
            let target = markdown_destination(raw);
            if !target.is_empty() && !is_external_target(target) {
                links.push(MarkdownLink {
                    label: visible_label(&line[open + 1..close]),
                    written_target: target.to_string(),
                    range: line_start + open..line_start + paren_end + 1,
                });
            }
            index = paren_end + 1;
        }
    }
    links
}

fn markdown_destination(raw: &str) -> &str {
    if let Some(stripped) = raw.strip_prefix('<') {
        if let Some(end) = stripped.find('>') {
            return &stripped[..end];
        }
    }
    raw.split_whitespace().next().unwrap_or("")
}

fn visible_label(label: &str) -> String {
    label
        .replace("\\[", "[")
        .replace("\\]", "]")
        .replace(['*', '`'], "")
}

fn is_external_target(target: &str) -> bool {
    if target.starts_with("//") {
        return true;
    }
    let Some(colon) = target.find(':') else {
        return false;
    };
    let scheme = &target[..colon];
    !scheme.is_empty()
        && scheme.chars().enumerate().all(|(index, ch)| {
            ch.is_ascii_alphabetic()
                || (index > 0 && (ch.is_ascii_digit() || ch == '+' || ch == '-' || ch == '.'))
        })
}

fn resolve_markdown_link(
    corpus_root: &Path,
    repository_root: &Path,
    document: &Document,
    link: &MarkdownLink,
    documents_by_path: &BTreeMap<PathBuf, &Document>,
) -> ResolvedLink {
    let decoded = percent_decode_minimal(&link.written_target);
    let (file_part, anchor) = split_anchor(&decoded);
    let lexical_target = if file_part.is_empty() {
        Some(document.path.clone())
    } else {
        clean_join(document.path.parent().unwrap_or(corpus_root), file_part)
    };

    let target_path = lexical_target
        .as_ref()
        .filter(|path| path.starts_with(repository_root));
    let resolved_path = target_path.map(|path| relative_display(repository_root, path));
    let resolved_target = resolved_path.as_ref().map(|path| {
        if anchor.is_empty() {
            path.clone()
        } else {
            format!("{path}#{anchor}")
        }
    });

    let mut target_locations = Vec::new();
    let mut resolved = false;
    if let Some(path) = target_path {
        if path.exists() {
            if anchor.is_empty() {
                target_locations.push(file_start_location(
                    repository_root,
                    path,
                    documents_by_path.get(path).copied(),
                ));
                resolved = true;
            } else if let Some(target_document) = documents_by_path.get(path).copied() {
                if let Some(location) = anchors_in(target_document).remove(anchor) {
                    target_locations.push(location);
                    resolved = true;
                }
            } else if path.extension() == Some(OsStr::new("md")) {
                if let Ok(target_document) = Document::read(repository_root, path) {
                    if let Some(location) = anchors_in(&target_document).remove(anchor) {
                        target_locations.push(location);
                        resolved = true;
                    }
                }
            }
        }
    }

    ResolvedLink {
        reference: GraphReference {
            syntax: "markdown_link",
            source: document.location(link.range.clone()),
            written_target: link.written_target.clone(),
            resolution: if resolved { "resolved" } else { "dangling" },
            target_locations,
            label: Some(link.label.clone()),
            resolved_target,
            normalized_target: None,
            scope: None,
        },
        target_path: resolved.then(|| target_path.cloned()).flatten(),
    }
}

fn file_start_location(
    repository_root: &Path,
    path: &Path,
    document: Option<&Document>,
) -> SourceLocation {
    document.map_or_else(
        || SourceLocation {
            path: relative_display(repository_root, path),
            start: SourcePosition { line: 1, column: 1 },
            end: SourcePosition { line: 1, column: 1 },
        },
        |document| document.location(0..0),
    )
}

fn anchors_in(document: &Document) -> BTreeMap<String, SourceLocation> {
    let mut anchors = BTreeMap::new();
    let mut seen = BTreeSet::new();
    for (line_start, line) in prose_lines(&document.content) {
        let trimmed = line.trim_start();
        let hashes = trimmed.bytes().take_while(|byte| *byte == b'#').count();
        if hashes == 0 || hashes > 6 || trimmed.as_bytes().get(hashes) != Some(&b' ') {
            continue;
        }
        let heading = trimmed[hashes..].trim();
        if heading.is_empty() {
            continue;
        }
        let base = github_anchor(heading);
        let mut anchor = base.clone();
        let mut suffix = 1;
        while seen.contains(&anchor) {
            anchor = format!("{base}-{suffix}");
            suffix += 1;
        }
        seen.insert(anchor.clone());
        anchors.insert(
            anchor,
            document.location(line_start..line_start + line.len()),
        );
    }
    anchors
}

fn github_anchor(heading: &str) -> String {
    let mut output = String::new();
    let mut last_dash = false;
    for ch in heading.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            output.push(ch);
            last_dash = false;
        } else if (ch.is_whitespace() || ch == '-') && !last_dash && !output.is_empty() {
            output.push('-');
            last_dash = true;
        }
    }
    output.trim_matches('-').to_string()
}

fn req_traces_in(document: &Document) -> Vec<ReqTrace> {
    let mut traces = Vec::new();
    for (line_start, line) in prose_lines(&document.content) {
        let bytes = line.as_bytes();
        let mut index = 0;
        while let Some(found) = line[index..].find("req").map(|offset| index + offset) {
            let before_ok = found == 0 || !is_word_byte(bytes[found - 1]);
            let mut cursor = found + 3;
            while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
                cursor += 1;
            }
            if before_ok && bytes.get(cursor) == Some(&b':') {
                cursor += 1;
                while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
                    cursor += 1;
                }
                let target_start = cursor;
                while bytes
                    .get(cursor)
                    .is_some_and(|byte| is_req_target_byte(*byte))
                {
                    cursor += 1;
                }
                while cursor > target_start && bytes[cursor - 1] == b'.' {
                    cursor -= 1;
                }
                if cursor > target_start {
                    traces.push(ReqTrace {
                        written: line[target_start..cursor].to_string(),
                        range: line_start + found..line_start + cursor,
                        target_range: line_start + target_start..line_start + cursor,
                    });
                }
            }
            index = found + 3;
        }
    }
    traces
}

fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn is_req_target_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/' | b'#')
}

fn resolve_req_trace(
    corpus_root: &Path,
    repository_root: &Path,
    document: &Document,
    trace: &ReqTrace,
    links: &[ResolvedLink],
    definitions_by_id: &BTreeMap<String, Vec<Definition>>,
) -> GraphReference {
    let (qualified_path, written_id) = trace
        .written
        .split_once('#')
        .map_or((None, trace.written.as_str()), |(path, id)| {
            (Some(path), id)
        });
    let normalized = written_id.to_ascii_uppercase();
    let (scope, targets) = if let Some(path) = qualified_path {
        let normalized_path = clean_join(repository_root, path)
            .filter(|target| target.starts_with(repository_root))
            .map(|target| relative_display(repository_root, &target));
        let targets = definitions_by_id
            .get(&normalized)
            .into_iter()
            .flatten()
            .filter(|definition| normalized_path.as_ref() == Some(&definition.artifact))
            .cloned()
            .collect::<Vec<_>>();
        (format!("artifact:{path}"), targets)
    } else {
        resolve_id_targets(corpus_root, document, &normalized, links, definitions_by_id)
    };
    reference_from_definitions(
        "req_trace",
        document.location(trace.range.clone()),
        trace.written.clone(),
        normalized,
        scope,
        targets,
    )
}

fn id_citations_in(document: &Document) -> Vec<IdOccurrence> {
    prose_lines(&document.content)
        .into_iter()
        .flat_map(|(line_start, line)| {
            ids_in_line(line).into_iter().map(move |mut occurrence| {
                occurrence.range =
                    line_start + occurrence.range.start..line_start + occurrence.range.end;
                occurrence
            })
        })
        .collect()
}

fn ids_in_line(line: &str) -> Vec<IdOccurrence> {
    let bytes = line.as_bytes();
    let mut occurrences = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if !bytes[index].is_ascii_uppercase()
            || (index > 0 && is_id_boundary_byte(bytes[index - 1]))
        {
            index += 1;
            continue;
        }

        if let Some(end) = qualified_id_end(bytes, index) {
            let written = &line[index..end];
            occurrences.push(IdOccurrence {
                written: written.to_string(),
                normalized: written.to_ascii_uppercase(),
                range: index..end,
            });
            index = end;
            continue;
        }
        if let Some(end) = local_id_end(bytes, index) {
            let written = &line[index..end];
            occurrences.push(IdOccurrence {
                written: written.to_string(),
                normalized: written.to_ascii_uppercase(),
                range: index..end,
            });
            index = end;
            continue;
        }
        index += 1;
    }
    occurrences
}

fn qualified_id_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut cursor = start + 1;
    while bytes
        .get(cursor)
        .is_some_and(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || *byte == b'.')
    {
        cursor += 1;
    }
    if bytes.get(cursor) != Some(&b'-') {
        return None;
    }
    let prefix = &bytes[start..cursor];
    if prefix.len() >= 2
        && matches!(prefix[0], b'A' | b'T' | b'R')
        && prefix[1..].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    local_id_end(bytes, cursor + 1)
}

fn local_id_end(bytes: &[u8], start: usize) -> Option<usize> {
    if !matches!(bytes.get(start), Some(b'A' | b'T' | b'R'))
        || !bytes.get(start + 1).is_some_and(u8::is_ascii_digit)
        || !bytes.get(start + 2).is_some_and(u8::is_ascii_digit)
    {
        return None;
    }
    let end = start + 3;
    (bytes
        .get(end)
        .is_none_or(|byte| !is_id_boundary_byte(*byte)))
    .then_some(end)
}

fn is_id_boundary_byte(byte: u8) -> bool {
    byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_'
}

fn resolve_id_citation(
    document: &Document,
    occurrence: IdOccurrence,
    links: &[ResolvedLink],
    definitions_by_id: &BTreeMap<String, Vec<Definition>>,
) -> GraphReference {
    let (scope, targets) = resolve_id_targets(
        Path::new(""),
        document,
        &occurrence.normalized,
        links,
        definitions_by_id,
    );
    reference_from_definitions(
        "id_citation",
        document.location(occurrence.range),
        occurrence.written,
        occurrence.normalized,
        scope,
        targets,
    )
}

fn resolve_id_targets(
    _corpus_root: &Path,
    document: &Document,
    normalized: &str,
    links: &[ResolvedLink],
    definitions_by_id: &BTreeMap<String, Vec<Definition>>,
) -> (String, Vec<Definition>) {
    let all = definitions_by_id
        .get(normalized)
        .cloned()
        .unwrap_or_default();
    if !is_local_id(normalized) {
        let namespace = normalized
            .rsplit_once('-')
            .map_or(normalized, |(prefix, _)| prefix);
        return (format!("namespace:{namespace}"), all);
    }

    if document.path.file_name() == Some(OsStr::new("requirements.md")) {
        return (
            format!("artifact:{}", document.repo_path),
            all.into_iter()
                .filter(|definition| definition.artifact == document.repo_path)
                .collect(),
        );
    }

    let requirement_artifacts = links
        .iter()
        .filter_map(|link| link.target_path.as_ref())
        .filter(|path| path.file_name() == Some(OsStr::new("requirements.md")))
        .map(|path| path.to_path_buf())
        .collect::<BTreeSet<_>>();
    let artifact_names = requirement_artifacts
        .iter()
        .filter_map(|path| {
            links.iter().find_map(|link| {
                (link.target_path.as_ref() == Some(path)).then(|| {
                    link.reference
                        .resolved_target
                        .as_deref()
                        .unwrap_or_default()
                        .split('#')
                        .next()
                        .unwrap_or_default()
                        .to_string()
                })
            })
        })
        .collect::<BTreeSet<_>>();
    let targets = all
        .into_iter()
        .filter(|definition| artifact_names.contains(&definition.artifact))
        .collect::<Vec<_>>();
    let scope = if artifact_names.is_empty() {
        "undeclared".to_string()
    } else {
        format!(
            "linked:{}",
            artifact_names.into_iter().collect::<Vec<_>>().join(",")
        )
    };
    (scope, targets)
}

fn is_local_id(id: &str) -> bool {
    let bytes = id.as_bytes();
    bytes.len() == 3
        && matches!(bytes[0], b'A' | b'T' | b'R')
        && bytes[1..].iter().all(u8::is_ascii_digit)
}

fn reference_from_definitions(
    syntax: &'static str,
    source: SourceLocation,
    written_target: String,
    normalized_target: String,
    scope: String,
    targets: Vec<Definition>,
) -> GraphReference {
    let target_locations = targets
        .iter()
        .map(|target| target.location.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    GraphReference {
        syntax,
        source,
        written_target,
        resolution: resolution(target_locations.len()),
        target_locations,
        label: None,
        resolved_target: None,
        normalized_target: Some(normalized_target),
        scope: Some(scope),
    }
}

fn ordinal_citations_in(
    document: &Document,
    companions_by_ordinal: &BTreeMap<String, Vec<Companion>>,
) -> Vec<OrdinalOccurrence> {
    let mut occurrences = Vec::new();
    for (line_start, line) in prose_lines(&document.content) {
        let bytes = line.as_bytes();
        let uppercase = line.to_ascii_uppercase();
        let upper = uppercase.as_bytes();

        let mut cursor = 0;
        while let Some(found) = uppercase[cursor..]
            .find("DELTA-")
            .map(|offset| cursor + offset)
        {
            let digit_start = found + 6;
            let digit_count = upper[digit_start..]
                .iter()
                .take_while(|byte| byte.is_ascii_digit())
                .count();
            let end = digit_start + digit_count;
            if digit_count > 0
                && (found == 0 || !is_word_byte(upper[found - 1]))
                && upper.get(end).is_none_or(|byte| !is_word_byte(*byte))
            {
                let written = &line[found..end];
                if let Some(normalized) = normalize_delta_ordinal(&line[digit_start..end]) {
                    occurrences.push(OrdinalOccurrence {
                        written: written.to_string(),
                        normalized,
                        range: line_start + found..line_start + end,
                    });
                }
            }
            cursor = end.max(found + 6);
        }

        for keyword in ["DECISION", "DECISIONS", "DEC."] {
            let mut cursor = 0;
            while let Some(found) = uppercase[cursor..]
                .find(keyword)
                .map(|offset| cursor + offset)
            {
                let before_ok = found == 0 || !is_word_byte(upper[found - 1]);
                let mut ordinal_start = found + keyword.len();
                while upper
                    .get(ordinal_start)
                    .is_some_and(|byte| byte.is_ascii_whitespace() || matches!(*byte, b'-' | b'`'))
                {
                    ordinal_start += 1;
                }
                let ordinal_end = ordinal_start + 4;
                if before_ok
                    && upper
                        .get(ordinal_start..ordinal_end)
                        .is_some_and(|digits| digits.iter().all(u8::is_ascii_digit))
                    && upper
                        .get(ordinal_end)
                        .is_none_or(|byte| !byte.is_ascii_digit() && *byte != b'-')
                {
                    let written = &line[ordinal_start..ordinal_end];
                    occurrences.push(OrdinalOccurrence {
                        written: written.to_string(),
                        normalized: written.to_string(),
                        range: line_start + ordinal_start..line_start + ordinal_end,
                    });
                }
                cursor = found + keyword.len();
            }
        }

        let mut cursor = 0;
        while let Some(open) = line[cursor..].find('`').map(|offset| cursor + offset) {
            let start = open + 1;
            let end = start + 4;
            if bytes
                .get(start..end)
                .is_some_and(|digits| digits.iter().all(u8::is_ascii_digit))
                && bytes.get(end) == Some(&b'`')
            {
                let written = &line[start..end];
                if companions_by_ordinal.contains_key(written) {
                    occurrences.push(OrdinalOccurrence {
                        written: written.to_string(),
                        normalized: written.to_string(),
                        range: line_start + start..line_start + end,
                    });
                }
                cursor = end + 1;
            } else {
                cursor = start;
            }
        }
    }
    occurrences.sort_by_key(|occurrence| (occurrence.range.start, occurrence.range.end));
    occurrences
        .dedup_by(|left, right| left.range == right.range && left.normalized == right.normalized);
    occurrences
}

fn resolve_ordinal_citation(
    corpus_root: &Path,
    repository_root: &Path,
    document: &Document,
    occurrence: OrdinalOccurrence,
    links: &[ResolvedLink],
    companions_by_ordinal: &BTreeMap<String, Vec<Companion>>,
    companion_by_path: &BTreeMap<PathBuf, Companion>,
) -> GraphReference {
    let linked = links
        .iter()
        .filter_map(|link| link.target_path.as_ref())
        .filter_map(|path| companion_by_path.get(path))
        .filter(|companion| companion.normalized_ordinal == occurrence.normalized)
        .map(|companion| (companion.repo_path.clone(), companion.clone()))
        .collect::<BTreeMap<_, _>>();
    let (scope, targets) = if !linked.is_empty() {
        let targets = linked.into_values().collect::<Vec<_>>();
        let paths = targets
            .iter()
            .map(|target| target.repo_path.clone())
            .collect::<Vec<_>>();
        (format!("linked:{}", paths.join(",")), targets)
    } else {
        resolve_ordinal_in_ancestors(
            corpus_root,
            repository_root,
            document,
            &occurrence.normalized,
            companions_by_ordinal,
        )
    };
    let target_locations = targets
        .into_iter()
        .map(|target| target.location)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    GraphReference {
        syntax: "ordinal_citation",
        source: document.location(occurrence.range),
        written_target: occurrence.written,
        resolution: resolution(target_locations.len()),
        target_locations,
        label: None,
        resolved_target: None,
        normalized_target: Some(occurrence.normalized),
        scope: Some(scope),
    }
}

fn resolve_ordinal_in_ancestors(
    corpus_root: &Path,
    repository_root: &Path,
    document: &Document,
    ordinal: &str,
    companions_by_ordinal: &BTreeMap<String, Vec<Companion>>,
) -> (String, Vec<Companion>) {
    let mut directory = document.path.parent().unwrap_or(corpus_root);
    if matches!(
        directory.file_name().and_then(OsStr::to_str),
        Some(".decisions" | ".delta")
    ) {
        directory = directory.parent().unwrap_or(corpus_root);
    }
    loop {
        let targets = companions_by_ordinal
            .get(ordinal)
            .into_iter()
            .flatten()
            .filter(|companion| companion.owner == directory)
            .cloned()
            .collect::<Vec<_>>();
        if !targets.is_empty() {
            return (
                format!("ancestor:{}", relative_display(repository_root, directory)),
                targets,
            );
        }
        if directory == corpus_root {
            break;
        }
        let Some(parent) = directory.parent() else {
            break;
        };
        if !parent.starts_with(corpus_root) {
            break;
        }
        directory = parent;
    }
    ("ancestor:none".to_string(), Vec::new())
}

fn resolution(target_count: usize) -> &'static str {
    match target_count {
        0 => "dangling",
        1 => "resolved",
        _ => "ambiguous",
    }
}

fn prose_lines(content: &str) -> Vec<(usize, &str)> {
    let mut lines = Vec::new();
    let mut in_fence: Option<char> = None;
    let mut offset = 0;
    for segment in content.split_inclusive('\n') {
        let line = segment.strip_suffix('\n').unwrap_or(segment);
        let trimmed = line.trim_start();
        let fence = if trimmed.starts_with("```") {
            Some('`')
        } else if trimmed.starts_with("~~~") {
            Some('~')
        } else {
            None
        };
        if let Some(marker) = fence {
            if in_fence == Some(marker) {
                in_fence = None;
            } else if in_fence.is_none() {
                in_fence = Some(marker);
            }
        } else if in_fence.is_none() {
            lines.push((offset, line));
        }
        offset += segment.len();
    }
    lines
}

fn overlaps_any(range: &Range<usize>, ranges: &[Range<usize>]) -> bool {
    ranges
        .iter()
        .any(|other| range.start < other.end && other.start < range.end)
}

fn split_anchor(target: &str) -> (&str, &str) {
    target.split_once('#').unwrap_or((target, ""))
}

fn percent_decode_minimal(value: &str) -> String {
    value.replace("%20", " ")
}

fn clean_join(base: &Path, relative: &str) -> Option<PathBuf> {
    if relative.is_empty() {
        return Some(base.to_path_buf());
    }
    let relative = Path::new(relative);
    if relative.is_absolute() {
        return None;
    }
    let mut output = base.to_path_buf();
    for component in relative.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => output.push(part),
            Component::ParentDir => {
                output.pop();
            }
            Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    Some(output)
}

fn relative_display(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn first_line_end(content: &str) -> usize {
    content.find('\n').unwrap_or(content.len())
}
