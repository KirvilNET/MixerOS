<script setup lang="ts">

</script>

<template>
  <div class="flex">
    <div class="flex h-screen w-16 flex-col justify-between border-e border-[#ff005c] bg-[#121212]">
      <div class="">
        <div class="flex items-center justify-center p-2">
          <img src="/Icon.svg" class="h-12 w-12">
        </div>

        <div class="border-t border-[#ff005c]">
          <div class="px-2">
            <div class="py-4">
              <a href="#" class="t group relative flex justify-center rounded-sm bg-[#282828] px-2 py-1.5 text-white">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
	                <path fill="currentColor" d="M14 9q-.425 0-.712-.288T13 8V4q0-.425.288-.712T14 3h6q.425 0 .713.288T21 4v4q0 .425-.288.713T20 9zM4 13q-.425 0-.712-.288T3 12V4q0-.425.288-.712T4 3h6q.425 0 .713.288T11 4v8q0 .425-.288.713T10 13zm10 8q-.425 0-.712-.288T13 20v-8q0-.425.288-.712T14 11h6q.425 0 .713.288T21 12v8q0 .425-.288.713T20 21zM4 21q-.425 0-.712-.288T3 20v-4q0-.425.288-.712T4 15h6q.425 0 .713.288T11 16v4q0 .425-.288.713T10 21z" />
                </svg>

                <span class="invisible absolute start-full top-1/2 ms-4 -translate-y-1/2 rounded-sm bg-gray-900 px-2 py-1.5 text-xs font-medium text-white group-hover:visible">
                  Dashboard
                </span>
              </a>
            </div>

            <ul class="space-y-1 border-t border-gray-100 pt-4">
              <li>
                <a href="#" class="group relative flex justify-center rounded-sm px-2 py-1.5 text-white hover:bg-[#282828] hover:text-white-200">
                  <svg xmlns="http://www.w3.org/2000/svg" class="size-5 opacity-75" viewBox="0 0 24 24">
                    <path fill="currentColor" d="M4 18q-.825 0-1.412-.587T2 16V5q0-.825.588-1.412T4 3h16q.825 0 1.413.588T22 5v11q0 .825-.587 1.413T20 18h-3l.7.7q.15.15.225.338t.075.387V20q0 .425-.288.712T17 21H7q-.425 0-.712-.288T6 20v-.575q0-.2.075-.387T6.3 18.7L7 18zm0-2h16V5H4zm0 0V5z"/>
                  </svg>

                  <span class="invisible absolute start-full top-1/2 ms-4 -translate-y-1/2 rounded-sm bg-gray-900 px-2 py-1.5 text-xs font-medium text-white group-hover:visible">
                    Clients
                  </span>
                </a>
              </li>

              <li>
                <a href="#" class="group relative flex justify-center rounded-sm px-2 py-1.5 text-white hover:bg-[#282828] hover:text-white-200">
                  <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
                  	<g fill="none">
                  		<circle cx="5" cy="5" r="3" stroke="currentColor" stroke-width="1.5" />
                  		<circle cx="19" cy="19" r="3" stroke="currentColor" stroke-width="1.5" />
                  		<path fill="currentColor" d="M11 4.25a.75.75 0 0 0 0 1.5zM13 19l.53.53a.75.75 0 0 0 0-1.06zm4.206-10.313l.402.633zM6.794 15.313l.403.632zm5.236 1.657a.75.75 0 0 0-1.06 1.06zm-1.06 3a.75.75 0 0 0 1.06 1.06zm-.567-6.064a.75.75 0 0 0-.806-1.266zm2.797-3.559a.75.75 0 0 0 .806 1.266zm2.932-6.097H11v1.5h5.132zM13 18.25H7.868v1.5H13zm.53.22l-1.5-1.5l-1.06 1.06l1.5 1.5zm-1.06 0l-1.5 1.5l1.06 1.06l1.5-1.5zm-4.602-.22c-1.25 0-1.726-1.633-.671-2.305l-.805-1.265c-2.321 1.477-1.275 5.07 1.476 5.07zm8.264-12.5c1.25 0 1.726 1.633.671 2.305l.805 1.265c2.321-1.477 1.275-5.07-1.476-5.07zm-6.535 6.89l-3.205 2.04l.805 1.265l3.206-2.04zm7.206-4.585L13.2 10.347l.806 1.266l3.602-2.293z" />
                  	</g>
                  </svg>

                  <span class="invisible absolute start-full top-1/2 ms-4 -translate-y-1/2 rounded-sm bg-gray-900 px-2 py-1.5 text-xs font-medium text-white group-hover:visible">
                    Routing
                  </span>
                </a>
              </li>

            </ul>
          </div>
        </div>
      </div>

      <div class="sticky inset-x-0 bottom-0 bg-[#121212] p-2">

        <a href="#" class="group relative flex w-full justify-center rounded-sm px-2 py-1.5 text-white hover:bg-[#282828] hover:text-white-200">
          <svg xmlns="http://www.w3.org/2000/svg" class="size-5 opacity-75" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
          </svg>
          <span class="invisible absolute start-full top-1/2 ms-4 -translate-y-1/2 rounded-sm bg-gray-900 px-2 py-1.5 text-xs font-medium text-white group-hover:visible">
            Settings
          </span>
        </a>

      </div>
    </div>
  </div>
</template>