# Rust Crash Course — Agent Guide

## Purpose

Continue this as a stateful, one-lesson-per-day Rust course. Produce one polished, interactive HTML lesson only when the user asks for the next day or asks to revise the current day.

## Before authoring a lesson

Read these, in order:

1. `MISSION.md` and `NOTES.md`
2. `learning-records/` to establish what the learner has actually demonstrated
3. The exact next course source in this folder (`03-introduction.md`, then `04-rust-tools.md`, and so on)
4. `RESOURCES.md` only for clearly labelled optional supplementation
5. `templates/lesson.html` and `assets/course.css`

The local chapter exports determine the sequence and core content. Visible attribution must credit [Tech With Tim’s Rust Programming Tutorial](https://www.youtube.com/playlist?list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ). External material may add useful context or practice, but it must be labelled **Optional extra** and must not replace the chapter’s teaching path.

## Current state

- Day 1 is complete: `lessons/0001-rust-introduction.html`.
- Evidence is in `learning-records/0001-first-rust-binary.md`.
- The next source is `04-rust-tools.md`; do not create it until the user asks to move on.

## Lesson contract

Create each lesson as `lessons/NNNN-dash-case-title.html`, copied from `templates/lesson.html` and linked to `../assets/course.css`.

Every lesson must contain:

- A concise outcome tied to the mission
- A visible Tech With Tim course attribution and link
- A **Zed Lab**: open the practice folder, use Zed’s integrated terminal, save, and run the exact command
- A small practice task with a tangible result
- One immediate-feedback retrieval check or other tight feedback loop
- A finish line that says exactly what evidence the learner should return
- Optional extras clearly separated from the source chapter

Write semantic HTML, not Markdown inside HTML. Keep each page short enough to complete in one sitting. Every page must be mobile-friendly: use the shared responsive CSS, ensure code blocks scroll rather than overflow, collapse multi-column layouts on narrow screens, and keep text, controls, images, and diagram labels within their containers. Preserve the course’s clean editorial visual system by reusing `assets/course.css`; do not duplicate or replace its palette with generic styling. Make new diagrams with ordinary accessible HTML/SVG only when they improve understanding, and keep all labels inside their bounds.

## Learning state

Create a new `learning-records/NNNN-name.md` only after the learner demonstrates an understanding or completes a meaningful task. Do not treat mere exposure to a lesson as learning. Add glossary terms only after the learner can use them accurately.

## Completion check

Before handing off, verify that the new page opens, links resolve, the Zed Lab command matches the lesson, the attribution is present, and all practice instructions lead to an observable result.

## Publish each completed class

After a requested lesson or lesson revision is complete, run `git status` and review the changed files. Commit only the intended course artifacts with a descriptive message, then push the current `main` branch to `origin`. Do this after the class is finished, not mid-lesson; preserve generated Rust binaries and other ignored build output outside commits.
