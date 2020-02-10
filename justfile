set shell := ["powershell.exe", "-c"]

build:
    cargo run

changes:
    git log --pretty=format:%s | Out-File -FilePath CHANGELOG.md