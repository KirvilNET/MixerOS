
type processor = {
    isLocal: boolean
    name: string
    id: number
    IP: string
    port: number
}

class DSP {
    private processorConfig: processor

    constructor(config: processor) {
        this.processorConfig = config
    }
}

export default DSP