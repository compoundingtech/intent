# intent

A documentation contract for the durable intent behind a system — what it is for,
what constrains it, what shape it takes, and why each of those was decided that
way.

Most projects already write this down somewhere. It ends up spread across README
prose, stale design docs, PR descriptions, chat history, and the heads of the
people who were there. The problem is not that the knowledge is missing; it is
that nothing says which of those places is authoritative, so none of them can be
trusted, and none can be checked.

`context/intent/` is that contract, specified in itself. The corpus in this repository
uses its own conventions to define its own conventions — `context/intent/vision.md` is a
vision document written to the rules that `context/intent/01-vision/` specifies.

## What it is for

Intent artifacts are **durable and timeless**. They describe the system as it is
meant to be, not the work in progress. A plan, a milestone, a ticket, and a status
update are all explicitly out of scope — they have a lifecycle measured in days,
and mixing them into the durable layer is what makes design docs rot.

The contract's main job is answering one question mechanically: **which artifact
owns this fact?** A constraint that must hold belongs in `requirements.md`. The
shape that satisfies it belongs in `spec.md`. Why that shape rather than another
belongs in a decision record. The gap between what is specified and what is true
right now belongs in a `.delta/` entry. Once each fact has exactly one home,
"is this still true?" becomes a question you can ask a file rather than a person.

The second job is making room for the things that are not yet durable truth.
Open questions, experiments, external references, and known drift each get a
lifecycle of their own, so unfinished thinking has somewhere to live that is not
the middle of the specification.

## Where to start

Read in this order:

1. **`context/intent/vision.md`** — what this is and what it deliberately is not.
2. **`context/intent/requirements.md`** — the constraints the contract holds itself to.
3. **`context/intent/spec.md`** — how the artifacts fit together.
4. **`context/intent/ontology.md`** — the vocabulary. Worth reading early; the terms are
   used precisely and the precision is the point.

Then follow whichever numbered section matches the artifact you care about.

## How the corpus is organised

The root holds the corpus's own intent — `vision.md`, `requirements.md`,
`spec.md`, `ontology.md`, `intuition.md`, `roadmap.md`. The numbered directories
each specify **one artifact type** in the contract, and each is itself a small
intent tree with its own `requirements.md` and `spec.md`.

| Section | Specifies |
| --- | --- |
| `01-vision` | `vision.md` — purpose and non-goals |
| `02-requirements` | `requirements.md` — testable constraints, assumptions, tradeoffs |
| `03-spec` | `spec.md` — the realised shape that satisfies the requirements |
| `04-ontology` | `ontology.md` — the system's vocabulary |
| `05-intuition` | `intuition.md` — the mental model behind the design |
| `06-decisions` | `.decisions/` — decision records and their lifecycle |
| `07-experiments` | `.experiments/` — investigations and their evidence |
| `08-reference` | `.reference/` — captured external source material |
| `09-delta` | `.delta/` — known divergence between spec and reality |
| `10-open-questions` | `open-questions.md` — what is deliberately unsettled |
| `11-roadmap` | `roadmap.md` — directions under consideration, not a plan |
| `12-common-subsystems` | recurring subsystem shapes worth naming once |
| `13-review-smells` | what a reviewer should look for in an intent tree |
| `14-grill-intent` | the procedure for interrogating a tree until it holds up |
| `15-evaluation` | isolated evaluation runs and their fixtures |
| `16-enforcement` | the rules a checker can enforce mechanically |

Three directories are hidden by convention and easy to miss — `.decisions/`,
`.delta/`, and `.experiments/`. Between them they hold a large share of the
corpus, including the reasoning behind most of what the numbered sections assert.
Use `ls -a`, and note that many tools skip them by default.

`context/` is separate: it is this repository's own intent tree, covering the
checker rather than the methodology.

## Two things worth knowing early

**The conventions need no toolchain.** They are plain Markdown with a naming
discipline: a directory layout, a set of filenames, and rules about which file
owns which fact. Authoring and reading them takes a text editor and nothing
else. This corpus is its own worked example — the conventions it describes are
the conventions it is written in, so every rule it states can be seen applied
in the files you are already reading.

**The rules are mechanically checkable, and checked.** `16-enforcement` is not
aspirational — it defines concrete rules with stable identifiers, and a real
checker implements them: decision-record shape, delta shape, proposed-decision
lifecycle, link targets, reference shape, experiment shape. This corpus passes
that checker under its strict profile, and dangling internal links or malformed
decision records fail it. The checker's own contract — its requirements, spec, and
decisions — is in `context/cli`, and its implementation is being brought into this
repository alongside it. Enforcement is deliberately
consumer-agnostic — the rules are defined so that any tool can implement them,
rather than binding the contract to one.

That combination is the argument for the whole thing. Conventions that cannot be
checked decay into folklore; conventions that need a bespoke toolchain do not get
adopted. This is an attempt at the narrow path between the two.
