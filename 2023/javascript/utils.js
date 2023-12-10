import { open } from 'node:fs/promises'
import { join } from 'node:path'

export async function withLines(fileName, handleLine, initialValue) {
    const file = await open(join('..', '_input', `${fileName}.txt`))
    try {
        let value = initialValue
        let index = 0
        for await (const line of file.readLines()) {
            value = handleLine(value, line, index++)
        }
        return value
    } finally {
        await file.close()
    }
}
