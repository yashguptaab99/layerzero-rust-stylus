const { ethers } = require('ethers')

async function runContractFunction() {
    const provider = new ethers.JsonRpcProvider('https://sepolia-rollup.arbitrum.io/rpc')
    const wallet = new ethers.Wallet('832ec1f374e2bb64ee5876f2a6ec571d6e27e665ff60f5fba8ab8baf10119cb9', provider)
    const contractAddress = '0xa085c6300fd3c29ff8ef5c582b9b867b4c1a2519'
    const contractABI = [
		{
			"inputs": [],
			"name": "InvalidDepositAmount",
			"type": "error"
		},
		{
			"inputs": [],
			"name": "InvalidEndpoint",
			"type": "error"
		},
		{
			"inputs": [
				{
					"internalType": "string",
					"name": "",
					"type": "string"
				}
			],
			"name": "PropagationError",
			"type": "error"
		},
		{
			"inputs": [
				{
					"internalType": "uint256",
					"name": "quote",
					"type": "uint256"
				}
			],
			"name": "deposit",
			"outputs": [],
			"stateMutability": "payable",
			"type": "function"
		},
		{
			"inputs": [
				{
					"internalType": "address",
					"name": "user",
					"type": "address"
				}
			],
			"name": "getBalance",
			"outputs": [
				{
					"internalType": "uint256",
					"name": "",
					"type": "uint256"
				}
			],
			"stateMutability": "view",
			"type": "function"
		},
		{
			"inputs": [],
			"name": "getCrossChainMessenger",
			"outputs": [
				{
					"internalType": "address",
					"name": "",
					"type": "address"
				}
			],
			"stateMutability": "view",
			"type": "function"
		},
		{
			"inputs": [
				{
					"internalType": "address",
					"name": "messenger_address",
					"type": "address"
				}
			],
			"name": "init",
			"outputs": [],
			"stateMutability": "nonpayable",
			"type": "function"
		}
	]
		
    const contract = new ethers.Contract(contractAddress, contractABI, wallet)

    // const functionName = 'getBalance'
    // const functionArgs = ['0x733b4d2Fab2E995c51a1BA0241a6bD1Ee71dD4C5']

    // const functionName = 'getCrossChainMessenger'
    // const functionArgs = []
    
    // try {
    //     const result = await contract[functionName](...functionArgs)
    //     console.log('Transaction successful, result:', result)
    // } catch (error) {
    //     console.error('Error executing contract function:', error)
    // }

    const depositAmount = ethers.parseEther('0.0001')
    const quote = ethers.parseEther('0.04')

    try {
        // await contract.init("0x125c0393158b0fD7BAD78425dBc2668a01A770d1");

        const tx = await contract.deposit(quote, { value: depositAmount + quote });
        console.log('Transaction sent, waiting for confirmation...')
        const receipt = await tx.wait() 
        console.log('Transaction confirmed, receipt:', receipt)
    } catch (error) {
        if (error.data) {
            const iface = new ethers.Interface(contractABI)
            const decodedError = iface.parseError(error.data)

            console.error('Custom Error Thrown:', decodedError.name)

            if (decodedError.name === 'InvalidDepositAmount') {
                console.error('Error: Invalid deposit amount.')
            } else if (decodedError.name === 'InvalidEndpoint') {
                console.error('Error: Invalid endpoint.')
            } else if (decodedError.name === 'PropagationError') {
                const errorMessage = decodedError.args[0];
                console.error('Error: Propagation issue occurred. Message:', errorMessage)
            }
        } else {
            console.error('Unknown Error:', error)
        }
    }
}

runContractFunction()
