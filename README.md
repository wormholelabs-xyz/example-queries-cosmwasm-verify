## Queries Verification on Cosmwasm PoC

This is a demo of verifying and parsing [Wormhole Queries](https://wormhole.com/queries/) on Cosmwasm.

This example uses the Wormhole Core contract to get the current guardian set.

Since the core contract does not expose a function to verify signatures against a payload, this example replicates
that code from the `parse_and_verify_vaa` core contract method in `verify_signatures_from_hash`.

This example also uses the `wormhole-query-sdk-rust` repo from Wormhole Labs to parse the query responses.

It provides a trivial example of parsing a query response for a `totalSupply` call on the WETH contract.

## Running the Tests

To run the tests do:

```shell
$ cargo test
```

## Building the Artifact

To build the wasm file for deployment do:

```shell
$ scripts/build.sh
```

## Testing in Tilt

You can play with this contract in the tilt environment using the sample query program.

### Deploying to Tilt

Before deploying this contract to tilt, do the following:

- Build the artifact as described in the previous step.
- Bring up tilt with the `--terra2` option.
- Check the `terra2-terrad` output in tilt and verify the address in the `Instantiated cw_wormhole.wasm at` line matches
  the value of `wormhole_contract` in `scripts/deploy_to_tilt.sh`. Update the script if necessary.

The first time you do the deployment, you need to do:

```shell
$ cd scripts
$ npm ci
```

Then you can deploy the contract by doing:

```shell
$ script/node deploy_to_tilt.js
```

### Run the example query

Once you have deployed the contract, you can run the sample query by doing the following.

First, verify the value of the `contract` in `scripts/query.js` matches the deployed address in the previous step. Update the script if necessary.

Then run the script by doing:

```shell
$ scripts/node query.js
```

---

⚠ **This software is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
implied. See the License for the specific language governing permissions and limitations under the License.**
