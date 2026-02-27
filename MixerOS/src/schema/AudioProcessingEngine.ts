/**
 * Type Definition for an audio Input
 *
 * @remarks 
 * 
 *
 * @param name - friendly name for the Input
 * @param id - programatic identifier for the Input
 * @param channel - The Channel of the Input on the Interface
 * 
 *
 * 
 */
type Input = {
    name: string
    id: number
    channel: string
}

/**
 * Type Definition for an audio Output
 *
 * @remarks 
 * 
 *
 * @param name - friendly name for the Output
 * @param id - programatic identifier for the Output
 * @param channel - The Channel of the Output on the Interface
 * 
 *
 * 
 */
type Output = {
    name: string
    id: number
    channel: string
}

/**
 * Type Definition For an Interface
 *
 * @remarks This a main aggergate for a Audio Interface and handles its Inputs and Outputs
 * 
 *
 * @param name - Friendly Name for the Interface
 * @param id - The programatic id for the Interface
 * @param Inputs - The Inputs of the Interface as a Input[]
 * @param Outputs - The Outputs of the Interface as an Output[]
 * @param Bitdepth - The Bit depth of the Interface
 * @param SampleRate - The Sample Rate of the Interface
 * 
 * 
 * 
 */
type Interface = {
    name: string
    id: number
    Inputs: Input[]
    Outputs: Output[]
    Bitdepth: number
    SampleRate: number
}

/**
 * The Class responsible for interfacing with the backend Sound Engine
 * 
 * 
 */
class SoundEngine {

    private Inputs: Input[]
    private Outputs: Output[]

    private AvailableInterfaces: Interface[]

    constructor() {
        this.AvailableInterfaces = []
        this.Inputs = []
        this.Outputs = []
    }

    /**
     * 
     * @returns The Current Inputs of the Sound engine as an Input[]
     */
    getInputs() {
        if (this.Inputs.length > 0) {
            return null
        } else {
            return this.Inputs
        }
    }

    /**
     * 
     * @returns The Current Outputs of the Sound engine as an Output[]
     */
    getOutputs() {
        if (this.Outputs.length > 0) {
            return null
        } else {
            return this.Outputs
        }
    }

    /**
     * 
     * @returns The Current Interfaces of the Sound engine as an Interface[]
     */
    getInterfaces() {
        if (this.AvailableInterfaces.length > 0) {
            return null
        } else {
            return this.AvailableInterfaces
        }
    }
}

export default SoundEngine;