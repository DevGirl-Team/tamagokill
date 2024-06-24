# ADR 1: Adoption of Architectural Decision Records

## Context
Tamagokill is a new project that will require a lot of decisions to be made on a technical aspect, and such decisions can currently be made by any contributing member of the project.

An Architectural Decision Records (ADR) is a small **markdown file** that documents a decision.

## Decision

Architectural decisions can now be proposed by anyone through a pull request containing only an ADR file without additionnal code. The decision is considered approved and taken once the pull request is merged. An approved decision remains in effect until it is revoked by another ADR. ADR file can't be modified, as they represent decision taken in the past.

Making an ADR is **not mandatory**. But they are recommended to take important complex decisions that will affect the project.

If needed the pull request that contains the ADR can include a temporary proof of concept (in a side branch, gist, pastebin, ...).

ADR can include assets, like images or diagrams files. Any assets must be referenced and accessible in the markdown file.

ADR markdown files follow the naming convention `./adr/<ISO date>-<ADR reference number>-<ADR title in kebab-case>.md`.

Asset files follow the naming convention `./adr/assets/<ISO date>-<ADR reference number>-<ADR title in kebab-case>.<asset name>.<asset extension>`.

Like any pull requests, ADRs needs a reasonnable number of reviews before being merged.

We adopt the following format for Architectural Decision Records (ADR):

- **Title**: A descriptive and concise title for the ADR.
- **Context**: A summary of the context in which the decision was made.
- **Decision**: The architectural decision made and it's full description.
- **Reasons**: The reasons behind the decision.
- **Implications**: The implications refer to the effects of this architectural decision on the existing project.

Unapproved ADRs will only exist as closed pull requests and won't be added to the project.

## Reasons
- **📝 Document upcoming and previous decisions**: Developpers should follow the evolution of the project and be up to date with best practices / tools / design patterns. Reading the ADRs history is a good way to keep up with the project.
- **🤝 Gathering consensus**: Every contributing member should be confortable with new aspects of the project, and can express their opinion before it is officialized.
- **🌱 Facilitate the evolution of the project**: It is sometimes hard to make complex changes or to experiment new things in fear of others opinions. ADR is a good way to give it a go !
-  **👥 Avoid necessity for an organisationnal tool / including everyone**: Decisions can be discussed directly in github, meaning there is no need for an additionnal medium (discord, slack, forum, ...) for supporting the project. This also mean decision making is not exclusive to those who are not part of those medium (no "places of power").
- **💖 Solving conflicts**: Devs have to follow taken decisions once it's agreed, sorting out divergences of opinions.

## Implications
There is no impact on the project for this first ADR.
