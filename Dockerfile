FROM debian:latest

#Params
ARG DATABASE_URL
ARG MOUNT_PATH
ARG DISK_STORAGE_DIRECTORY

#Environment
ENV DATABASE_URL=${DATABASE_URL}
ENV MOUNT_PATH=${MOUNT_PATH}
ENV DISK_STORAGE_DIRECTORY=${DISK_STORAGE_DIRECTORY}

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