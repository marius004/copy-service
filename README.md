# CopyService

CopyService is a user-space daemon and library designed to facilitate asynchronous file copying on a local machine.
It allows users to create, cancel, pause, and monitor copy jobs efficiently.

## Features
- Asynchronous copying of files.
- Daemon with configurable settings.
- Job management functionalities:
  - Create a new copy job.
  - Cancel a copy job.
  - Pause a copy job.
  - Retrieve progress and status of a copy job.
  - List all existing copy jobs.
  - 
## Configuration
The daemon can be configured using a local [config](https://github.com/marius004/copy-service/blob/master/Config.toml) file.
This file specifies the maximum number of threads the daemon can use and the maximum number of concurrent jobs it can handle.
