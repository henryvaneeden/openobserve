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
  <q-card class="column full-height">
    <q-card-section class="q-px-md q-py-md">
      <div class="row items-center no-wrap">
        <div class="col">
          <div v-if="beingUpdated" class="text-body1 text-bold">
            {{ t("dashboard.updatedashboard") }}
          </div>
          <div v-else class="text-body1 text-bold">
            {{ t("dashboard.createdashboard") }}
          </div>
        </div>
        <div class="col-auto">
          <q-btn v-close-popup="true" round flat icon="cancel" />
        </div>
      </div>
    </q-card-section>
    <q-separator />
    <q-card-section class="q-w-md q-mx-lg">
      <q-form ref="addDashboardForm" @submit.stop="onSubmit.execute">
        <q-input
          v-if="beingUpdated"
          v-model="dashboardData.id"
          :readonly="beingUpdated"
          :disabled="beingUpdated"
          :label="t('dashboard.id')"
        />
        <q-input
          v-model="dashboardData.name"
          :label="t('dashboard.name') + ' *'"
          color="input-border"
          bg-color="input-bg"
          class="q-py-md showLabelOnTop"
          data-test="dashboard-name"
          stack-label
          outlined
          filled
          dense
          :rules="[(val) => !!val.trim() || t('dashboard.nameRequired')]"
          :lazy-rules="true"
        />
        <span>&nbsp;</span>
        <q-input
          v-model="dashboardData.description"
          :label="t('dashboard.typeDesc')"
          color="input-border"
          bg-color="input-bg"
          class="q-py-md showLabelOnTop"
          stack-label
          outlined
          filled
          dense
        />

        <span>&nbsp;</span>
        <!-- select folder or create new folder and select -->
        <select-folder-dropdown @folder-selected="selectedFolder = $event" />

        <div class="flex justify-center q-mt-lg">
          <q-btn
            v-close-popup="true"
            class="q-mb-md text-bold"
            :label="t('dashboard.cancel')"
            text-color="light-text"
            padding="sm md"
            no-caps
          />
          <q-btn
            data-test="dashboard-add-submit"
            :disable="dashboardData.name.trim() === ''"
            :loading="onSubmit.isLoading.value"
            :label="t('dashboard.save')"
            class="q-mb-md text-bold no-border q-ml-md"
            color="secondary"
            padding="sm xl"
            type="submit"
            no-caps
          />
        </div>
      </q-form>
    </q-card-section>
  </q-card>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import dashboardService from "../../services/dashboards";
import { useI18n } from "vue-i18n";
import { useStore } from "vuex";
import { isProxy, toRaw } from "vue";
import { getImageURL } from "../../utils/zincutils";
import { convertDashboardSchemaVersion } from "@/utils/dashboard/convertDashboardSchemaVersion";
import SelectFolderDropdown from "./SelectFolderDropdown.vue";
import { getAllDashboards } from "@/utils/commons";
import { useQuasar } from "quasar";
import { useLoading } from "@/composables/useLoading";

const defaultValue = () => {
  return {
    id: "",
    name: "",
    description: "",
  };
};

let callDashboard: Promise<{ data: any }>;

export default defineComponent({
  name: "ComponentAddDashboard",
  props: {
    activeFolderId: {
      type: String,
      default: "default",
    },
  },
  emits: ["updated"],
  setup(props, { emit }) {
    const store: any = useStore();
    const beingUpdated: any = ref(false);
    const addDashboardForm: any = ref(null);
    const disableColor: any = ref("");
    const dashboardData: any = ref(defaultValue());
    const isValidIdentifier: any = ref(true);
    const { t } = useI18n();
    const $q = useQuasar();
    const activeFolder: any = store.state.organizationData.folders.find(
      (item: any) => item.folderId === props.activeFolderId
    );
    const selectedFolder = ref({
      label: activeFolder.name,
      value: activeFolder.folderId,
    });

    //generate random integer number for dashboard Id
    function getRandInteger() {
      return Math.floor(Math.random() * (9999999999 - 100 + 1)) + 100;
    }

    const onSubmit = useLoading(async () => {
      await addDashboardForm.value.validate().then(async (valid: any) => {
        if (!valid) {
          return false;
        }

        const dashboardId = dashboardData.value.id;
        delete dashboardData.value.id;

        if (dashboardId == "") {
          const obj = toRaw(dashboardData.value);
          const baseObj = {
            title: obj.name,
            // NOTE: the dashboard ID is generated at the server side,
            // in "Create a dashboard" request handler. The server
            // doesn't care what value we put here as long as it's
            // a string.
            dashboardId: "",
            description: obj.description,
            role: "",
            owner: store.state.userInfo.name,
            created: new Date().toISOString(),
            panels: [],
            version: 2,
          };

          callDashboard = dashboardService.create(
            store.state.selectedOrganization.identifier,
            baseObj,
            selectedFolder.value.value ?? "default"
          );
        }
        await callDashboard
          .then(async (res: { data: any }) => {
            const data = convertDashboardSchemaVersion(
              res.data["v" + res.data.version]
            );

            //update store
            await getAllDashboards(store, selectedFolder.value.value);
            emit("updated", data.dashboardId, selectedFolder.value.value);
            dashboardData.value = {
              id: "",
              name: "",
              description: "",
            };
            await addDashboardForm.value.resetValidation();
          })
          .catch((err: any) => {
            $q.notify({
              type: "negative",
              message: JSON.stringify(
                err.response.data["error"] || "Dashboard creation failed."
              ),
            });
          });
      });
    });

    return {
      t,
      disableColor,
      isPwd: ref(true),
      beingUpdated,
      status,
      dashboardData,
      addDashboardForm,
      store,
      getRandInteger,
      isValidIdentifier,
      getImageURL,
      selectedFolder,
      onSubmit,
    };
  },
  methods: {
    onRejected(rejectedEntries: string | any[]) {
      this.$q.notify({
        type: "negative",
        message: `${rejectedEntries.length} file(s) did not pass validation constraints`,
      });
    },
  },
  components: { SelectFolderDropdown },
});
</script>
