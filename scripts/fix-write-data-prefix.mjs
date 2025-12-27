// @ts-check
import { bottomUpTransformerVisitor, assertIsNode, numberTypeNode } from 'codama';

export default function () {
    return bottomUpTransformerVisitor([
        {
            select: '[instructionNode]write.[instructionArgumentNode]bytes',
            transform: node => {
                assertIsNode(node, 'instructionArgumentNode');
                assertIsNode(node.type, 'sizePrefixTypeNode');
                return { ...node, type: { ...node.type, prefix: numberTypeNode('u64') } };
            },
        },
    ]);
}
