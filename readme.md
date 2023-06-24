# Simple file sharing backend
This is the backend for my simple file sharing service that I made for my final project, in order to graduate from my university. <br>

<br>

# Tech Stack
1. Backend code written in [`Rust`](https://www.rust-lang.org/).
2. Connects to [`PostgreSQL`](https://www.postgresql.org/)
3. Modeling using [`Prisma`](https://www.prisma.io/)
4. File stored on [`AWS S3`](https://aws.amazon.com/s3/)
5. Local S3 [`Minio`](https://min.io/)

<br>

# Features list ( implemented )

## User: 
- [x] See all users
- [x] Create new user
- [x] Update user
- [x] Delete user

## Folders:
- [x] See all public folders
- [x] See all personal folders
- [x] See all "shared to me" folders
- [x] Create new folders
- [x] Update folders
- [x] Delete folders
- [ ] Manage folder's collaborators
- [ ] Download folder
- [ ] Manage folders' tags

## Files:
- [x] See all public files
- [x] See all personal files
- [x] See all "shared to me" files
- [x] Create new files
- [x] Update files
- [x] Delete files
- [x] Manage file versions
- [ ] Manage file's collaborators
- [x] Download file
- [x] File extensions unrestricted
- [ ] Manage files' tags

## Tags
- [ ] Create new tags
- [ ] Delete tags
- [ ] Manage tags of files or folders

## Collaboration
- [ ] Add collaborator to files
- [ ] Add collaborator to folders
- [ ] Delete collaborators from files
- [ ] Delete collaborators from folders

<br>

# How to deploy locally

### 1. Adjust some variables in the [`docker-compose.prod.yml`](./docker-compose.prod.yml) based on your likings 
### 2. Run this command:
```zsh
docker-compose -f docker-compose.prod.yml up -d
```
### 3. Wait for the `simple-file-sharing` container to finish building