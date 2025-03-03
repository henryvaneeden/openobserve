<!-- Copyright 2023 Zinc Labs Inc.

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http:www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License. 
-->

<template>
  <div class="q-mt-lg">
    <div class="tags-title text-bold q-mb-sm q-ml-xs">
      Session Replay
    </div>
    <div class="row">
      <template v-for="(value, tag) in getSessionTags" :key="tag.tag">
        <ErrorTag :tag="{ key: tag, value }" />
      </template>
    </div>
    <div
      class="play-button bg-primary cursor-pointer q-mt-md text-white row items-center"
      style="width: fit-content; border-radius: 3px; padding: 6px 8px"
      @click="playSessionReplay"
    >
      <q-icon name="play_circle" size="18px" class="q-mr-xs" /> Play Session
      Replay
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import ErrorTag from "./ErrorTag.vue";
import { useRouter } from "vue-router";

const props = defineProps({
  error: {
    type: Object,
    required: true,
  },
});

const router = useRouter();

const getSessionTags = computed(() => {
  return {
    session_id: props.error.session_id,
    view_id: props.error.view_id,
  };
});

const playSessionReplay = () => {
  router.push({
    name: "SessionViewer",
    params: {
      id: props.error.session_id,
    },
    query: {
      start_time: props.error._timestamp,
      end_time: props.error._timestamp,
    },
  });
};
</script>

<style scoped>
.tags-title {
  font-size: 16px;
}
</style>
