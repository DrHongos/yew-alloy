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
  return chainId
} 

// try to get the connected status persistent
// implement all metamask requests!