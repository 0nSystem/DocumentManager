FROM debian:latest

ENV DATABASE_URL="postgres://postgres:postgres@localhost/postgres"
ENV MOUNT_PATH="/static"
ENV DISK_STORAGE_DIRECTORY="/assets"

WORKDIR /app

SHELL ["/bin/bash", "-c"]

# Install Tool dependencies
RUN apt-get update && apt-get install -y curl
#Download Latest Release
RUN curl -s https://api.github.com/repos/0nSystem/DocumentManager/releases/latest  \
    | grep "browser_download_url"  \
    | grep "linux" \
    | cut -d '"' -f 4  \
    | xargs curl -L -o document_manager_latest_release
#Add execute permission
RUN chmod +x ./document_manager_latest_release
ENTRYPOINT ["./document_manager_latest_release"]