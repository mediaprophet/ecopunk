# Native project operator walkthrough

This is a source-based walkthrough of the saved native controls. It is not a record of a GUI session, OS credential-store operation, human enrollment, or accepted production work.

## Open the native project shell

From Poet's native command-line package, run:

```text
poet studio --native-project <existing-project-directory>
```

The directory locates bounded public configuration. Opening it does not adopt the authority, unlock a profile, open HMC source, or grant a role. The optional `--check-project` mode inspects public configuration only and does not launch the window.

For the saved RUN10 public-configuration-only check, the source-linked executable alias is `target/debug/poet-batch5-2e7ab1b867bf.exe`. The executed artifact is recorded in [artifact.json](../direct-cli/2e7ab1b867bf/artifact.json) (SHA-256 `5f29320457b05c0567025361fb228280882d0ea01cae6e2c99d44ed8f0a0c091`), and its result is in [native-project-check.json](../direct-cli/native-project-check.json). This records a CLI check only; the alias was not launched into the GUI.

## Create a new local project

In **Poet · Cooperative Project**, fill **New project name**, **Local controller profile label**, and **Consent / enrollment / decision evidence reference**. The profile label is a local locator; the UI's **Person capacity requested (self-declared; project verification is separate)** checkbox records only a requested capacity. Before creation, select:

> Create a new local project authority and explicitly admit this controller as its steward. This grants no authority in other projects.

Then choose **Create project and secure steward profile**. This creates a new local project authority and an editable `working.hmc`; it does not adopt an external project, verify a person's associated identifiers or qualification, generate an accepted master, or grant reviewer status. Existing project configuration cannot be replaced through this action.

## Adopt an existing project

When a project is found, compare the displayed **Project authority public-key fingerprint** with the authority you intend to trust. Select:

> I have checked this authority fingerprint and choose to adopt it for this project.

Then choose **Adopt this project authority**. Adoption opens the signed authority state but does not itself authenticate a member or load project HMC source.

## Enroll a contributor and make a separate review grant

After adoption, enter a local profile label and choose **Create this local secure controller** to create a controller profile through the configured native credential provider. A profile stores public locator metadata; its private key stays in the native protected store. No CLI/MCP argument can create a steward or reviewer role.

For a contributor request, enter the profile, a **Consent / enrollment / decision evidence reference**, and the requested capacity, then choose **Request contributor enrollment**. The key proof establishes controller-key possession. It does not establish personhood, human capacity, the validity or scope of associated identifiers, or contributor qualification.

An admitted steward signs in and uses **Steward enrollment decisions**. Enter a decision evidence reference. For a request that asks for person capacity, also enter **Human verification evidence reference, only for a person**; person capacity remains a separately evidenced steward decision; the reference does not establish personhood or the validity, issuer, scope, or authority of associated identifiers. Choose **Approve this enrollment** or **Decline this request** for the pending request. Admission changes lock current sessions, so sign in again afterward.

Reviewer authority is separate. On the member row, use **Grant explicit source-review authority** with its own decision evidence. This signed admission change also locks the current session; sign in again before continuing. Steward key revocation/recovery uses a separate procedure.

## Sign in and work on the accepted source

Enter the admitted profile under **Local controller profile label**, set **Working HMC filename** (the default is `working.hmc`), and choose **Unlock and sign in**. The host proves a fresh session challenge against current admission before it opens HMC source. To leave the current actor, use **Lock and switch contributor**; select the next admitted profile and choose **Unlock and sign in**. Each login uses a fresh session and worker lease.

Poet shows the **Vibe source draft** as a private draft. Accepted source is replaced in the draft only through the explicit **Load current accepted source into the local draft** panel: inspect its accepted-source preview, select **Replace my local draft with this accepted source; unsubmitted local text will be replaced.**, then choose **Replace local draft with accepted source**. This confirmation only replaces the local draft; it does not edit accepted source or record a review decision. Existing actor-private draft text is retained until the operator confirms replacement.

Use **Check** to validate the draft and **Submit source** to submit it through the current source/session authority. If execution needs reconciliation, use the explicitly offered **Rebuild execution session** action; source is not resubmitted automatically. For a new filename, **Save accepted source as new HMC** creates a new file and does not overwrite existing `working.hmc`. To update the project's existing `working.hmc`, open **HMC resources**, set **HMC resource package filename** to `working.hmc`, then choose **Inspect resources**. Confirm the displayed **Inspected package snapshot SHA256** for that exact file and current source/session; the save action is bound to this inspected digest and is rejected if the package or authority changed. Then choose **Save accepted source into this HMC**. Treat the resulting package write record as an audit of the write.

In **Cooperative source work**, choose **Inspect current source, history and review evidence** before recording. **Record contribution to this exact source** binds an effort/status record to the displayed admitted source revision. The effort/status form does not value effort as money or verify payment. Choose **Request review of displayed source revision** to request review of that exact current revision; drafts are not included in the request.

## Review as a distinct admitted reviewer

Use **Lock and switch contributor**, then **Unlock and sign in** with a different admitted profile that has an explicit current source-review grant. Inspect the exact source and evidence with **Inspect current source, history and review evidence**. Under **Distinct reviewer decision**, choose the exact current request, provide a decision identifier, observed rationale and inspection evidence, then select **Approve source only**, **Request source changes**, or **Reject source**. A current reviewer grant and a different reviewer subject are checked when committing the decision.

**Approve source only** approves only that exact source revision. It does not accept package resources, HMC meaning, rights, a complete model, or a high-grade master. Software-agent review remains software review; the source UI does not record human acceptance.

## Qualification limits

This route and its labels are source-backed. No GUI was launched for this walkthrough, and no real OS credential, person, trust anchor, contributor decision, or reviewer decision was used. Native OS credential-provider persistence, real project adoption, human verification, graphical contributor/reviewer usability, full fresh-process HMC/source recovery, complete HMC/model review, and accepted high-grade production remain separate open outcomes.

