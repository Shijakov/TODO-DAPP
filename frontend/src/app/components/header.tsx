import { FC } from 'react'

import ChainInfo from './chin-info'
import NavMenu from './nav-menu'
import Wallet from './web3/wallet'

const Header: FC = () => {
  return (
    <div className="container relative flex justify-between">
      <ChainInfo />
      <NavMenu />
      <Wallet />
    </div>
  )
}

export default Header
