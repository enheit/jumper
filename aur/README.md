# AUR Package for Jumper

This directory contains the files needed to publish Jumper to the Arch User Repository (AUR).

## Publishing to AUR

### Prerequisites

1. Create an AUR account at https://aur.archlinux.org/register
2. Add your SSH public key to your AUR account
3. Install the required tools:
   ```bash
   sudo pacman -S base-devel git
   ```

### Steps to Publish

1. **Clone the AUR repository:**
   ```bash
   git clone ssh://aur@aur.archlinux.org/jumper-git.git
   cd jumper-git
   ```

2. **Copy the PKGBUILD and .SRCINFO files:**
   ```bash
   cp /path/to/jumper/aur/PKGBUILD .
   cp /path/to/jumper/aur/.SRCINFO .
   ```

3. **Update your information in PKGBUILD:**
   Edit the `Maintainer` line at the top of PKGBUILD with your name and email.

4. **Test the package locally:**
   ```bash
   makepkg -si
   ```

5. **Update .SRCINFO (if you made changes to PKGBUILD):**
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```

6. **Commit and push to AUR:**
   ```bash
   git add PKGBUILD .SRCINFO
   git commit -m "Initial commit" # or "Update to version X.Y.Z"
   git push
   ```

### Updating the Package

When you release a new version:

1. The `pkgver()` function automatically updates based on git commits
2. If you need to make changes to dependencies or build process, update PKGBUILD
3. Regenerate .SRCINFO: `makepkg --printsrcinfo > .SRCINFO`
4. Commit and push the changes

## Testing Locally

Before publishing, test the package:

```bash
# In the aur directory
makepkg -si

# This will:
# - Download sources
# - Build the package
# - Install it on your system
```

## AUR Guidelines

Make sure your package follows [AUR submission guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines):

- Package name should be lowercase
- Use `-git` suffix for VCS packages
- Include all necessary dependencies
- Test thoroughly before submitting

## Support

For AUR package issues, users should comment on the AUR page:
https://aur.archlinux.org/packages/jumper-git

For application issues, direct users to:
https://github.com/enheit/jumper/issues
