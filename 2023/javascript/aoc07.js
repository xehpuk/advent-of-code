import {withLines} from './utils.js'

const JOKER = 'J'

const cards = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']

const handTypes = [
    /(.)\1{4}/, // Five of a kind
    /(.)\1{3}/, // Four of a kind
    /(.)\1(?:\1(.)\2|(.)\3\3)/, // Full house
    /(.)\1\1/, // Three of a kind
    /(.)\1.?(.)\2/, // Two pair
    /(.)\1/, // One pair
    /./, // High card
].map((type, i, types) => [type.test.bind(type), types.length - i])

function calcCardStrengths(cards) {
    return new Map(cards.map((card, i, cards) => [card, cards.length - i]))
}

function calcHand(hand) {
    const normalizedHand = Array.from(hand).toSorted().join('')
    const [, strength] = handTypes.find(([handType]) => handType(normalizedHand))
    return strength
}

function calcTotalWinnings(hands) {
    return hands.toSorted((hand1, hand2) => {
        const strength1 = hand1.strength
        const strength2 = hand2.strength
        for (let i = 0; i < strength1.length; i++) {
            const cmp = strength2[i] - strength1[i]
            if (cmp !== 0) {
                return cmp
            }
        }
        return 0
    }).reduce((totalWinnings, {bid}, i, {length}) => totalWinnings + bid * (length - i), 0)
}

function createHandleLine(calcHand, cardStrengths) {
    return (hands, line) => {
        const [hand, bid] = line.split(' ')
        const strength = [
            calcHand(hand),
            ...(Array.from(hand).map(card => cardStrengths.get(card))),
        ]
        return [...hands, {strength, bid}]
    }
}

export function part1(fileName = '07') {
    return withLines(fileName, createHandleLine(calcHand, calcCardStrengths(cards)), []).then(calcTotalWinnings)
}

export function part2(fileName = '07') {
    const cardStrengths = calcCardStrengths(cards
        .filter(card => card !== JOKER)
        .concat(JOKER))

    function calcHand2(hand) {
        if (!hand.includes(JOKER)) {
            return calcHand(hand)
        }
        const [cardReplacement] = Object.entries(Array.from(hand)
            .filter(card => card !== JOKER)
            .reduce((cards, card) => ({
                ...cards,
                [card]: (cards[card] ?? 0) + 1,
            }), {})).reduce((prev, curr) => prev[1] >= curr[1] ? prev : curr, [JOKER, 0])
        return calcHand(hand.replaceAll(JOKER, cardReplacement))
    }

    return withLines(fileName, createHandleLine(calcHand2, cardStrengths), []).then(calcTotalWinnings)
}