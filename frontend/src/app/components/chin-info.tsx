import { FC, useEffect, useState } from 'react'

import { useInkathon } from '@scio-labs/use-inkathon'

const ChainInfo: FC = () => {
  const { api, activeChain } = useInkathon()
  const [chainInfo, setChainInfo] = useState<{ [_: string]: any }>()

  // Fetch Chain Info
  const fetchChainInfo = async () => {
    if (!api) {
      setChainInfo(undefined)
      return
    }

    const chain = (await api.rpc.system.chain())?.toString() || ''
    const version = (await api.rpc.system.version())?.toString() || ''
    const properties = ((await api.rpc.system.properties())?.toHuman() as any) || {}
    const tokenSymbol = properties?.tokenSymbol?.[0] || 'UNIT'
    const tokenDecimals = properties?.tokenDecimals?.[0] || 12
    const chainInfo = {
      Chain: chain,
      Version: version,
      Token: `${tokenSymbol} (${tokenDecimals} Decimals)`,
    }
    setChainInfo(chainInfo)
  }
  useEffect(() => {
    fetchChainInfo()
  }, [api])

  // Connection Loading Indicator
  if (!api) {
    return (
      <div>
        Connecting to {activeChain?.name} ({activeChain?.rpcUrls?.[0]})
      </div>
    )
  }

  return (
    <div>
      {Object.entries(chainInfo || {}).map(([key, value]) => (
        <div key={key}>
          {key}:<strong>{value}</strong>
        </div>
      ))}
    </div>
  )
}

export default ChainInfo
