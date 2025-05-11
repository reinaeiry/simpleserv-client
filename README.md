# SimpleServ Client

## 🖥️ Host Script Integration

For full functionality, the **SimpleServ Client** is intended to be used with the **simpleserv-host** script. This script runs on the host machine and receives system information from the client.

You can find the **host script** repository here: [simpleserv-host](https://github.com/reinaeiry/simpleserv-host).

This script allows you to, as long as you are on the same network as your device:

- Monitor CPU and memory usage.
- View uptime and temperature information.
- Control the system (power off, restart).

---

## 🚀 Features

- **System Status Monitoring**: View detailed information about CPU, memory, uptime, and temperature.
- **Power Controls**: Power off or restart the system with a simple button click.
- **Run Command**: Execute system commands remotely and receive output.
- **Graphing**: Real-time graphs showing CPU and memory usage over time. (Dependent on use of the host script; [simpleserv-host](https://github.com/reinaeiry/simpleserv-host)!)
- **Standalone Support**: Can be used with Postman or similar apps for returning information directly.
  
## 📦 Requirements

- Rust (preferably the latest stable version)
- A running server that exposes status information (e.g., `simpleserv-host`)

## 🔧 Installation

1. **Clone the repository**:
   
   First, clone the repository to your local machine:

   ```
   git clone https://github.com/reinaeiry/simpleserv-client.git
   cd simpleserv-client
   ```

2. **Build the project**:
   
   Use `cargo` to build the project:

   ```
   cargo build --release
   ```

3. **Run the project**:
   
   Start the server by running:

   ```
   cargo run
   ```

   The server will start running on `http://localhost:3030` by default.

---

## 🌐 API Endpoints

### 1. **System Status (`/status`)**
   
   This endpoint fetches the system status information including:
   - Hostname
   - Uptime
   - CPU Usage
   - Memory Usage
   - Temperature (Not currently Implemented)

   **Method**: `GET`

   **Response**:

   ```
   {
     "hostname": "MySystem",
     "uptime": 123456,
     "cpu_usage": 45.7,
     "used_memory": 2048,
     "total_memory": 8192,
     "temperature": N/A
   }
   ```

### 2. **Execute Command (`/exec`)**
   
   This endpoint allows you to execute a command on the system. You can pass the command as a query parameter.

   **Method**: `POST`

   **Parameters**:
   - `cmd`: The command to run on the system.

   Example URL:

   ```
   http://localhost:6969/exec?cmd=echo+%2Fhi
   ```

   **Response**: The output of the executed command.

   ```
   hi
   ...
   ```
   
   **Json Format:**
   ```
   URL = http://192.168.1.105:3030/exec
   { "cmd": "echo hi" }
   ```
   
   

### 3. **Power Off & Restart**

   - **Power Off**: This triggers a shutdown process on the system. `localhost:3030/power/shutdown`
   - **Restart**: This triggers a restart on the system. `localhost:3030/power/restart`

---

## 🧰 Usage with Postman (or similar apps)

You can interact with the client API directly using tools like **Postman**. Here's how you can test some of the functionality:

1. **System Status (`/status`)**:
   
   Make a `GET` request to `http://localhost:3030/status`. The response will contain the system status.

2. **Execute Command (`/exec`)**:
   
   Make a `POST` request with a `cmd` query parameter, like `http://localhost:3030/exec?cmd=ipconfig+%2Fall`.

---

## ⚙️ Intended Use

This client is designed to work seamlessly when paired with the **host script**. While you can use the API directly with tools like Postman, for a more fluid experience, it's recommended to use it with the host script for interactive GUI elements.
