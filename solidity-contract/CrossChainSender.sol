// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import { OAppSender, MessagingFee } from "@layerzerolabs/oapp-evm/contracts/oapp/OAppSender.sol";
import { OptionsBuilder } from "@layerzerolabs/oapp-evm/contracts/oapp/libs/OptionsBuilder.sol";
import { OAppCore } from "@layerzerolabs/oapp-evm/contracts/oapp/OAppCore.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

contract CrossChainSender is OAppSender {
    using OptionsBuilder for bytes;
    bytes _options = OptionsBuilder.newOptions().addExecutorLzReceiveOption(50000, 0);

    address public rustContract;

    event MessageSent(string message, uint32 dstEid);

    constructor(address _endpoint, address _rustContract) OAppCore(_endpoint, msg.sender) Ownable(msg.sender) {
        require(_rustContract != address(0), "Invalid Rust contract address");
        rustContract = _rustContract;
    }

    modifier onlyRustContract() {
        require(msg.sender == rustContract, "Not authorized");
        _;
    }

    function setRustContract(address _rustContract) external onlyOwner {
        require(_rustContract != address(0), "Invalid address");
        rustContract = _rustContract;
    }

    function quote(
        uint32 _dstEid,
        string memory _message,
        bool _payInLzToken
    ) public view returns (MessagingFee memory fee) {
        bytes memory payload = abi.encode(_message);
        fee = _quote(_dstEid, payload, _options, _payInLzToken);
    }

    function send(
        uint32 _dstEid,
        string memory _message
    ) external payable onlyRustContract {
        bytes memory _encodedMessage = abi.encode(_message);
        _lzSend(
            _dstEid,
            _encodedMessage,
            _options,
            MessagingFee(msg.value, 0),
            payable(msg.sender)
        );

        emit MessageSent(_message, _dstEid);
    }
}
