# GitHub Wiki Update Instructions

The `wiki/` directory contains 18 markdown files staged for migration to the GitHub Wiki. These files cover SigmaOS system components:

- Boot-Configuration.md
- Container-Runtime.md
- Filesystem-Management.md
- Hardware-Detection.md
- Init-System.md
- Logging-System.md
- Memory-Management.md
- Monitoring.md
- Network-Management.md
- Power-Management.md
- Process-Management.md
- Recovery.md
- Security.md
- Service-Management.md
- Storage-Management.md
- System-Configuration.md
- Update-Management.md
- User-Management.md

## Manual Wiki Initialization

GitHub Wikis must be initialized manually via the GitHub UI:

1. Go to https://github.com/AaryanSinghChauhan09/Sigma/wiki
2. Click "Create the first page" if the wiki is empty
3. For each file in `wiki/`:
   - Create a new page with the same filename (without .md)
   - Copy the content from the corresponding `wiki/` file
   - Save the page

## Automated Wiki Updates (Future)

Once the wiki is initialized, future updates can be automated via:
- GitHub CLI: `gh wiki create <file> --repo AaryanSinghChauhan09/Sigma`
- Or a script that syncs `wiki/` to the GitHub Wiki
