set shell := ["powershell.exe", "-c"]

changes:
    git log --pretty=format:%s | Out-File -FilePath CHANGELOG.md