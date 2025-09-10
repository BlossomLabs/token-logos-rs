## token-logos-rs

Lightweight HTTP service (Rust + Spin) that resolves and redirects to token logo URLs from CoinGecko token lists. Useful when you need a consistent, cacheable endpoint for token images without bundling assets yourself.

### Features
- Redirects to official CoinGecko logo URLs via HTTP 302
- Supports multiple EVM chains via configurable mapping
- Zero address special-case for native tokens (ETH / POL)
- Minimal dependencies, compiled to `wasm32-wasip1` and run with Spin

### Prerequisites
- Rust 1.78+ (`rustup` recommended)
- Target: `wasm32-wasip1`
- Spin CLI installed

```bash
rustup target add wasm32-wasip1
```

Install Spin: see the official docs at `https://spinframework.dev/`

### Build & Run (local)
Use Spin to build the WebAssembly component and start the HTTP server:

```bash
spin up --build
```

Default address: `http://127.0.0.1:3000`

### API
#### GET `/`
Returns a short usage string.

#### GET `/token/{chain_id}/{address}`
- Looks up `{address}` in the CoinGecko token list for the mapped network of `{chain_id}`
- If found, responds `302 Found` with the logo URL in the `Location` header
- If not found, responds `404 Not Found`

##### Special cases
- If `{address}` equals the zero address `0x0000000000000000000000000000000000000000`:
  - For `chain_id` 137 (Polygon), redirects to the Polygon logo (POL)
  - For any other supported chain, redirects to the Ethereum logo (ETH)

##### Notes
- Address matching is case-insensitive
- The response is a redirect; clients should follow the `Location` URL to fetch the image

### Configuration
Supported chains are configured via the `SUPPORTED_CHAINS` environment variable, defined in `spin.toml`:

```toml
[component.token-logos-rs.environment]
SUPPORTED_CHAINS = "ethereum:1,optimistic-ethereum:10,arbitrum-one:42161,base:8453,polygon-pos:137,unichain:130,linea:59144"
```

Format: `network_id:chain_id` pairs, comma-separated. Example additions:

```text
bsc:56,avalanche:43114
```

At runtime, the service maps the numeric `{chain_id}` from the URL to the corresponding CoinGecko token list network id and fetches:

```
https://tokens.coingecko.com/{network_id}/all.json
```

To change defaults, edit `spin.toml` or override the environment when deploying.

### Examples
Assuming local server on port 3000:

```bash
# ETH native logo via zero address on Ethereum mainnet (chain_id 1)
curl -i http://127.0.0.1:3000/token/1/0x0000000000000000000000000000000000000000

# Polygon native logo via zero address on Polygon (chain_id 137)
curl -i http://127.0.0.1:3000/token/137/0x0000000000000000000000000000000000000000

# USDC on Ethereum mainnet
curl -i http://127.0.0.1:3000/token/1/0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48
```

A successful lookup returns `302 Found` with a `Location` header pointing to the image URL on `assets.coingecko.com` or a related host.

### Project Layout
- `src/routes/mod.rs`: request routing and HTTP responses
- `src/services/coingecko.rs`: fetch and parse CoinGecko token lists
- `src/config.rs`: parse `SUPPORTED_CHAINS` into a mapping
- `src/constants.rs`: common constants (zero address, default logo URLs)
- `src/models.rs`: minimal structs for token list JSON
- `spin.toml`: Spin application manifest, build command, routes, and environment

### Deployment
This app is a single Spin component. Common options:
- Local: `spin up --build`
- Container or edge runtimes that support Spin-compatible WASI components
- Fermyon Cloud: `spin deploy` (ensure outbound hosts in `spin.toml` and any env overrides are set)

Ensure outbound access to:
- `https://tokens.coingecko.com`
- `https://api.coingecko.com` (listed for future use)

### Error Handling
- `404 Not Found`: token not present in the list or no logo URL available
- `500 Internal Server Error`: network or JSON parsing errors when fetching the token list

### License
This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0).  
See [LICENSE](LICENSE) for details.

