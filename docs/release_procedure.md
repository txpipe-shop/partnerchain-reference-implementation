### Create a release branch

The release process involves a couple changes to the repository: updating the version number in Cargo.toml and adding a changelog.  In order to make these release-related changes, create a branch in your repository clone.  Note that all the examples below use `${R1}` as the release version, `${R0}` as the previous release, and `${R2}` as the next release.  You'll want to use the appropriate version numbers for the release you're working toward.

Set the R0, R1, and R2 variables.  For example:
```
R0=6.10.0 # previous release
R1=6.11.0 # this release
R2=6.11.1 # next release
```

Check out a new release branch:
```
git fetch partnerchain-reference-implementation
git checkout -b release-v${R1} partnerchain-reference-implementation/main
```

### Create a changelog

Create a changelog for all merges to `main` since the previous release.  You'll want to create a log for all changes since the previous release and name the resulting file like the target release.

```
echo "# ${R1}\n\n#### List of all changes\n\nSee below for a complete list of features and fixes.\n" > changelog/v${R1}.md
./tasks/changelog.sh v${R0}.. >> changelog/v${R1}.md
```

Edit the resulting changelog file with any summary notes or special upgrade considerations.

### Update `Cargo.toml`

Update the version numbers in each package's `Cargo.toml` to reflect the target release.

```toml
[package]
name = ...
version = {R1}
...
```

To pick this change up in `Cargo.lock`, run

```
cargo build
```

Commit the version updates including the node, runtime and other packages' Cargo.toml if applicable, and the new changelog:

```
git add node/Cargo.toml runtime/Cargo.toml Cargo.lock changelog
git commit -m "Updates for the ${R1} release"
```

### Pull request

Create a pull request for the release branch.  This allows for any final review of upgrade notes or other parts of the changelog. Do not merge yet!

```
git push partnerchain-reference-implementation release-v${R1}
```

### Tag

After CI jobs for the above pull request pass, tag the release:

```
# tag the latest from partnerchain-reference-implementation/release-v${R1}
git tag -a v${R1} -m "${R1}"

# push the tag
git push partnerchain-reference-implementation v${R1}
```

### Publish the draft GitHub release

Manually [create a draft release](https://github.com/txpipe-shop/partnerchain-reference-implementation/releases/new). Edit the release to update the notes, copying from the changelog created in the previous step.

### Merge the release branch

Give the pull request for the release branch a final review, and merge it into `main`.

### Patch releases

For a patch release, the process is basically the same as for a minor release. The significant difference is that you want to create a new branch from the minor tag.  For example:

```
git fetch --tags --prune partnerchain-reference-implementation
git checkout -b release-v3.20.1 v3.20.0
```

Next, you want to cherry pick commits that fix regressions in the minor release.

```
# for each bug fix commit
git cherry-pick <SHA_OF_BUG_FIX_COMMIT>
```

Since the `changelog.sh` relies on merge commits, and there won't be any merge commits between the previous release tag and your patch release branch, you'll need to manually create the changelog.  Copy one of the existing changelogs for a patch release as a starting point.  Link to the pull requests that included the original commits that you cherry picked above.

From this point, you can follow the normal release process ([after the changelog step](#update-cargotoml)).