<template>
  <v-container fluid class="text-left">
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
      {{ error }}
    </v-alert>

    <v-data-table
      :headers="headers"
      :items="devices"
      item-key="id"
      :loading="loading"
      loading-text="Loading... Please wait"
      hide-default-footer>
      <template v-slot:no-data>
        <p class="text-center">
          No device
        </p>
      </template>

      <template v-slot:item.createdAt="{ item }">
        <span>{{ item.createdAt | date }}</span>
      </template>
      <template v-slot:item.actions="{ item }">
        <v-btn
            outlined
            small
            color="success"
            :ripple="false"
            v-if="item.id === current.id">
              Current
          </v-btn>
          <v-btn
            v-else
            color="error"
            @click="revokeSession(item)"
            :loading="item.isLoading">
              Revoke
          </v-btn>
      </template>
      <template v-slot:item.device="{ item }">
        <span><v-icon>{{ getDeviceTypeIcon(item.device) }}</v-icon></span>
        <span><v-icon>{{ getDeviceOsIcon(item.device) }}</v-icon></span>
      </template>
    </v-data-table>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import * as models from '@/api/models';
import { RevokeSessionParams, Method } from '@/core/users';
import core from '@/core';

@Component
export default class DevicesTable extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) loading!: boolean;
  @Prop({ type: Array }) devices!: models.Session[];
  @Prop({ type: Object }) current!: models.Session;

  // data
  error = '';
  headers = [
    {
      sortable: true,
      text: 'Signed in',
      value: 'createdAt',
    },
    {
      sortable: false,
      text: 'Device',
      value: 'device',
    },
    {
      sortable: false,
      text: 'Actions',
      value: 'actions',
    },
  ];

  // computed
  // lifecycle
  // watch
  // methods
  async revokeSession(session: models.Session) {
    this.error = '';
    (session as any).isLoading = true; // eslint-disable-line
    const params: RevokeSessionParams = {
      id: session.id,
    };

    try {
      await core.call(Method.RevokeSession, params);
      this.$emit('revoked', session);
    } catch (err) {
      this.error = err.message;
    }
  }

  getDeviceTypeIcon(device: models.SessionDevice): string {
    switch (device.type) {
      case models.SessionDeviceType.Mobile:
        return 'mdi-cellphone';
      case models.SessionDeviceType.Console:
        return 'mdi-tablet-ipad';
      default:
        return 'mdi-laptop';
    }
  }

  getDeviceOsIcon(device: models.SessionDevice): string {
    switch (device.os) {
      case models.SessionDeviceOs.Linux:
        return 'mdi-linux';
      case models.SessionDeviceOs.Macos:
      case models.SessionDeviceOs.Ios:
        return 'mdi-apple';
      case models.SessionDeviceOs.Windows:
        return 'mdi-microsoft-windows';
      default:
        return 'mdi-android-debug-bridge';
    }
  }
}
</script>


<style lang="scss" scoped>
</style>
