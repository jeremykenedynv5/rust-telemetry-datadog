# Rust-telemetry-datadog

This project is a prototype to implement observervability in rust-actix-web framework. Trace are generated for each request and are stored in open-telemetry compatible services (Jaeger or Datadog).
## Prerequisites

To execute this example you need a running Jaeger instance.  
You can launch one using Docker:

```bash
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
```

## Running

You can launch this example with 

```bash
cargo run
```

An `actix-web` application will be listening on port `8000`.  
You can fire requests to it with:

```bash
curl -v http://localhost:8000/OptimusPrime
```
```text
Hello OptimusPrime!
```

## Traces

You can look at the exported traces in your browser by visiting [http://localhost:16686](http://localhost:16686).  
Spans will be also printed to the console in JSON format, as structured log records.

## Credits
This application uses Open Source components. You can find the source code of their open source projects along with license information below. I acknowledge and am grateful to these developers for their contributions to open source.

```
 Project: https://github.com/LukeMathWalker/tracing-actix-web/tree/main/examples/opentelemetry
 Copyright (c) 2022 LukeMathWalker
 License (MIT) https://github.com/LukeMathWalker/tracing-actix-web/blob/main/LICENSE-MIT
```

## License

This program is licensed under the "MIT License". Please see the file LICENSE in the source distribution of this software for license terms.