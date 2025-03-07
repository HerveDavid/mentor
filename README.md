```
::::    ::::  :::::::::: ::::    ::: :::::::::::  ::::::::  :::::::::  
+:+:+: :+:+:+ :+:        :+:+:   :+:     :+:     :+:    :+: :+:    :+: 
+:+ +:+:+ +:+ +:+        :+:+:+  +:+     +:+     +:+    +:+ +:+    +:+ 
+#+  +:+  +#+ +#++:++#   +#+ +:+ +#+     +#+     +#+    +:+ +#++:++#:  
+#+       +#+ +#+        +#+  +#+#+#     +#+     +#+    +#+ +#+    +#+ 
#+#       #+# #+#        #+#   #+#+#     #+#     #+#    #+# #+#    #+# 
###       ### ########## ###    ####     ###      ########  ###    ### 
```

# MENTOR

MENTOR (Model Electrical Network Training Operational Reality) is a Rust-based simulation orchestrator designed to model, analyze, and optimize electrical networks with a focus on training scenarios for operational reality.

## ⚠️ Important Warning

Please be aware of the following current limitations:

**Limited Format Support:**
- At present, MENTOR only supports IIDM (Internal Interface for Data Modeling) in JSON format. Additional formats may be supported in future releases.
- **Prototype Status:** This software is currently a prototype intended to test the Entity Component System (ECS) architecture. It should not be used in production environments without thorough testing and validation.

## Installation

### Prerequisites
- Rust (stable channel, 1.85.0 or newer)
- Cargo (included with Rust)

### Basic Usage

```bash
# Clone the repository
git clone https://github.com/HerveDavid/mentor.git
cd mentor

# Build the project
cargo build

# Run the application
cargo run
```

When you run the application with `cargo run`, it will start a web interface accessible at **http://localhost:3000**

### Loading IIDM Files

MENTOR currently only supports loading IIDM files in JSON format. Example files can be found in the repository at:
```
iidm/tests/data/network.json
```

### API Routes

You can interact with MENTOR through its API routes. These can be accessed using tools like cURL or Postman:

#### Streaming Component State Changes (GET)
This endpoint uses Server-Sent Events (SSE) to stream state changes for a specific component:

```bash
# Using cURL to stream state changes for a line
curl http://localhost:3000/api/iidm/stream/Line/NHV1_NHV2_2
```

In Postman, create a GET request to the URL: `http://localhost:3000/api/iidm/stream/Line/NHV1_NHV2_2`

#### Updating Component State (POST)
This endpoint allows you to modify the state of a component:

```bash
# Using cURL to update a line's state
curl -X POST http://localhost:3000/api/iidm/update/Line \
  -H "Content-Type: application/json" \
  -d '{"id": "NHV1_NHV2_2", "component": {"x": 1.0}}'
```

In Postman, create a POST request to the URL: `http://localhost:3000/api/iidm/update/Line` with a raw JSON body:
```json
{
  "id": "NHV1_NHV2_2",
  "component": {
    "x": 1.0
  }
}
```

## Contributing

We welcome contributions to MENTOR! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
