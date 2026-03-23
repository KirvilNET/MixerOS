<script setup lang="ts">

enum DeviceType {
    LocalIO,
    RemoteIO,
    LocalEngine,
    RemoteEngine
}


interface Device {
    name: String,
    id: number,
    type: DeviceType,
    inputs: number,
    outputs: number
}

const props = defineProps<{
    devices: Array<Device>
}>();

</script>

<template>
    <template v-if="devices.length == 0">
        <div class="w-full h-full flex justify-center items-center bg-[#21212121]">
            <div class="h-screen w-50 text-center">
                <h2 class="mt-6 text-2xl font-bold text-white"> No devices</h2>
                <p class="mt-4 text-pretty text-grey-200">
                    Make sure that the devices are online and working.
                </p>

                <button type="button" class="mt-6 block w-full rounded-lg bg-[#ff005c] px-6 py-3 text-sm font-medium text-white transition-colors hover:bg-[#ff637d]">
                    Refresh
                </button>
            </div>
        </div>
    </template>
    <template v-else>
        <div class="relative overflow-x-auto shadow-xs rounded-base border border-default bg-[#3f3f3f]">
            <table class="w-full text-sm text-left rtl:text-right text-body">
                <thead class="text-sm text-body border-b border-default-medium">
                    <tr>
                        <th scope="col" class="p-4">
                            <div class="flex items-center">
                                <input id="table-checkbox-38" type="checkbox" value=""
                                    class="w-4 h-4 border border-default-medium rounded-xs bg-neutral-secondary-medium focus:ring-2 focus:ring-brand-soft">
                                <label for="table-checkbox-38" class="sr-only">Table checkbox</label>
                            </div>
                        </th>
                        <th scope="col" class="px-6 py-3 font-medium text-white">
                            Device
                        </th>
                        <th scope="col" class="px-6 py-3 font-medium text-white">
                            Device Type
                        </th>
                        <th scope="col" class="px-6 py-3 font-medium text-white">
                            Inputs
                        </th>
                        <th scope="col" class="px-6 py-3 font-medium text-white">
                            Outputs
                        </th>
                        <th scope="col" class="px-6 py-3 font-medium text-white">
                            Connection
                        </th>
                        <th scope="col" class="px-6 py-3 font-medium text-white">
                            Status
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <template v-for="device in devices">
                        <tr class="border-b border-default">
                            <td class="w-4 p-4">
                                <div class="flex items-center">
                                    <input id="table-checkbox-39" type="checkbox" value=""
                                        class="w-4 h-4 border border-default-medium rounded-xs bg-neutral-secondary-medium focus:ring-2 focus:ring-brand-soft">
                                    <label for="table-checkbox-39" class="sr-only">Table checkbox</label>
                                </div>
                            </td>
                            <th scope="row" class="px-6 py-4 font-medium text-heading whitespace-nowrap">
                                {{ device.id }}
                            </th>
                            <td class="px-6 py-4">
                                {{ device.name }}
                            </td>
                            <td class="px-6 py-4">
                                {{device.type}}
                            </td>
                            <td class="px-6 py-4">
                                {{ device.inputs }}
                            </td>
                            <td class="px-6 py-4">
                                {{ device.outputs }}
                            </td>
                        </tr>
                    </template>
                </tbody>
            </table>
        </div>
    </template>
</template>