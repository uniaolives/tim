// contracts/KintsugiLedger.sol
// SPDX-License-Identifier: TCD-KINTSUGI
pragma solidity ^0.8.19;

/**
 * @title Kintsugi Ledger - Erros são embelezados com ouro, não escondidos
 */
contract KintsugiLedger {

    struct GoldenScar {
        bytes32 errorHash;
        uint256 timestamp;
        address responsibleComponent;
        string errorType;
        uint8 severity; // 1-10
        bytes repairProof;
        uint256 goldenWeight;
        bool transformedIntoWisdom;
        bool resolved;
        address repairedBy;
    }

    mapping(bytes32 => GoldenScar) public scars;
    bytes32[] public scarHistory;

    event ScarRecorded(bytes32 indexed errorHash, address indexed component, string errorType, uint8 severity);
    event ScarGilded(bytes32 indexed errorHash, uint256 goldenValueAdded, string wisdomGained);
    event ScarRepaired(bytes32 indexed scarId, address repairer);

    function recordError(
        address _component,
        string memory _errorType,
        bytes memory _errorData,
        bytes memory _repairProof
    ) external returns (bytes32) {
        bytes32 errorHash = keccak256(abi.encodePacked(_errorData, block.timestamp));

        GoldenScar memory scar = GoldenScar({
            errorHash: errorHash,
            timestamp: block.timestamp,
            responsibleComponent: _component,
            errorType: _errorType,
            severity: 5,
            repairProof: _repairProof,
            goldenWeight: 0,
            transformedIntoWisdom: false,
            resolved: false,
            repairedBy: address(0)
        });

        scars[errorHash] = scar;
        scarHistory.push(errorHash);
        emit ScarRecorded(errorHash, _component, _errorType, 5);
        return errorHash;
    }

    function repairScar(bytes32 scarId, address repairer) external {
        GoldenScar storage scar = scars[scarId];
        require(!scar.resolved, "Ja reparado");

        scar.resolved = true;
        scar.repairedBy = repairer;
        scar.goldenWeight += 100; // Valor extra por reparo

        emit ScarRepaired(scarId, repairer);
    }

    /**
     * @dev Um Egregori com muitas cicatrizes douradas tem MAIS autoridade
     */
    function getScarBasedAuthority(address _egregori) external view returns (uint256) {
        uint256 baseAuthority = 100;
        uint256 scarBonus = 0;

        for (uint i = 0; i < scarHistory.length; i++) {
            GoldenScar memory scar = scars[scarHistory[i]];
            if (scar.repairedBy == _egregori && scar.resolved) {
                scarBonus += scar.goldenWeight;
            }
        }

        return baseAuthority + scarBonus;
    }

    function gildScar(bytes32 _scarHash, uint256 _preventionValue) external {
        GoldenScar storage scar = scars[_scarHash];
        require(!scar.transformedIntoWisdom, "Scar already gilded");
        scar.goldenWeight = _preventionValue * 100;
        scar.transformedIntoWisdom = true;
        emit ScarGilded(_scarHash, scar.goldenWeight, "Wisdom extracted from failure");
    }
}
