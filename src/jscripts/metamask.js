//https://docs.metamask.io/wallet/reference/json-rpc-api/

export async function connect() {
  const accs = await ethereum
    .request({
      method: 'eth_requestAccounts',
      params: [],
    })
  //console.log('request accounts', accs)
  return accs
}
export async function getChainId() {
  const chainId = await ethereum
    .request({
      method: 'eth_chainId',
      params: [],
    })
  //console.log(`chain id ${chainId}`)
  return chainId
} 

// try to get the connected status persistent
// implement all metamask requests!


// also check https://github.com/MetaMask/metamask-sdk/blob/main/packages/examples/pure-javascript/index.html