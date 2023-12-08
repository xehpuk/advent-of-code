import {withLines} from './utils.js'

const cardStrengths = new Map(['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
    .map((card, i, cards) => [card, cards.length - i]))

const handTypes = [
    /(.)\1{4}/,
    /(.)\1{3}/,
    /(.)\1(?:\1(.)\2|(.)\3\3)/,
    /(.)\1\1/,
    /(.)\1.?(.)\2/,
    /(.)\1/,
    /./,
].map((type, i, types) => [type.test.bind(type), types.length - i])

function calcHandStrength(hand) {
    const normalizedHand = Array.from(hand).toSorted().join('')
    return [
        handTypes.find(([handType]) => handType(normalizedHand))[1],
        ...Array.from(hand).map(card => cardStrengths.get(card)),
    ]
}

export function part1(fileName = '07') {
    return withLines(fileName, (hands, line) => {
        const [hand, bid] = line.split(' ')
        const strength = calcHandStrength(hand)
        return [...hands, {strength, bid}]
    }, []).then(hands =>
        hands.toSorted((hand1, hand2) => {
            const strength1 = hand1.strength
            const strength2 = hand2.strength
            for (let i = 0; i < strength1.length; i++) {
                const cmp = strength2[i] - strength1[i]
                if (cmp !== 0) {
                    return cmp
                }
            }
            return 0
        }).reduce((totalWinnings, {bid}, i, {length}) => totalWinnings + bid * (length - i), 0))
}

export function part2(fileName = '07') {
    throw new Error('TODO')
}
