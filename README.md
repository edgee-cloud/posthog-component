<div align="center">
<p align="center">
  <a href="https://www.edgee.cloud">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.cloud/img/component-dark.svg">
      <img src="https://cdn.edgee.cloud/img/component.svg" height="100" alt="Edgee">
    </picture>
  </a>
</p>
</div>

<h1 align="center">PostHog component for Edgee</h1>

[![Coverage Status](https://coveralls.io/repos/github/edgee-cloud/posthog-component/badge.svg)](https://coveralls.io/github/edgee-cloud/posthog-component)
[![GitHub issues](https://img.shields.io/github/issues/edgee-cloud/posthog-component.svg)](https://github.com/edgee-cloud/posthog-component/issues)
[![Edgee Component Registry](https://img.shields.io/badge/Edgee_Component_Registry-Public-green.svg)](https://www.edgee.cloud/edgee/posthog-component)


This component enables seamless integration between [Edgee](https://www.edgee.cloud) and [PostHog](https://posthog.com/), allowing you to collect and forward analytics events to your posthog instance.


## Quick Start

1. Download the latest component version from our [releases page](../../releases)
2. Place the `posthog.wasm` file in your server (e.g., `/var/edgee/components`)
3. Add the following configuration to your `edgee.toml`:

```toml
[[destinations.data_collection]]
id = "posthog"
file = "/var/edgee/components/posthog.wasm"
settings.region = "YOUR_POSTHOG_REGION"
settings.api_key = "YOUR_API_KEY"
```


## Event Handling

### Event Mapping
The component maps Edgee events to PostHog Event as follows:

| Edgee Event | PostHog Event | Description |
|-------------|----------------|-------------|
| Page        | `$pageview` | pageview event with url as property |
| Track       | Your event name | A event with your custom event name |
| User        | `$identify` | Sets all the data for the user |


## Configuration Options

### Basic Configuration
```toml
[[destinations.data_collection]]
id = "posthog"
file = "/var/edgee/components/posthog.wasm"
settings.region = "YOUR_POSTHOG_REGION"
settings.api_key = "YOUR_API_KEY"
```


### Event Controls
Control which events are forwarded to posthog:
```toml
settings.edgee_page_event_enabled = true   # Enable/disable page view tracking
settings.edgee_track_event_enabled = true  # Enable/disable custom event tracking
settings.edgee_user_event_enabled = true   # Enable/disable user identification
```


## Development

### Building from Source
Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)
- wit-deps: `cargo install wit-deps`

Build command:
```bash
edgee component build
```

Test command:
```bash
make test
```

Test coverage command:
```bash
make test.coverage[.html]
```

### Contributing
Interested in contributing? Read our [contribution guidelines](./CONTRIBUTING.md)

### Security
Report security vulnerabilities to [security@edgee.cloud](mailto:security@edgee.cloud)