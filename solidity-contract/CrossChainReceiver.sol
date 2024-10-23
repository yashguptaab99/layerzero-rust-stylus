// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import { OAppReceiver, Origin } from "@layerzerolabs/oapp-evm/contracts/oapp/OAppReceiver.sol";
import { OAppCore } from "@layerzerolabs/oapp-evm/contracts/oapp/OAppCore.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

contract CrossChainReceiver is OAppReceiver {
    string public data = "Nothing received yet";

    event MessageReceived(string message, uint32 senderEid, bytes32 sender, uint64 nonce);

    constructor(address _endpoint) OAppCore(_endpoint, msg.sender) Ownable(msg.sender) {}

    function _lzReceive(
        Origin calldata _origin,
        bytes32,
        bytes calldata message,
        address,
        bytes calldata
    ) internal override {
        data = abi.decode(message, (string));
        emit MessageReceived(data, _origin.srcEid, _origin.sender, _origin.nonce);
    }
}
