import { open } from 'node:fs/promises'
import { join } from 'node:path'

export async function withLines(fileName, handleLine, initialValue) {
    const file = await open(join('..', '_input', `${fileName}.txt`))
    try {
        let value = initialValue
        for await (const line of file.readLines()) {
            value = handleLine(value, line)
        }
        return value
    } finally {
        await file.close()
    }
}
