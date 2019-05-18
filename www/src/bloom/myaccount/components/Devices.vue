<template>
    <v-data-table
    :headers="headers"
    :items="sessions"
    hide-actions
    class="elevation-0"
    disable-initial-sort
    >
    <template slot="items" slot-scope="props">
      <td class="text-xs-center">
        <div>
          <span>
          <v-icon small v-if="props.item.device.type === 'computer'">mdi-laptop</v-icon>
          <v-icon small v-else-if="props.item.device.type === 'tablet'">mdi-tablet-android</v-icon>
          <v-icon small v-else-if="props.item.device.type === 'phone'">mdi-cellhpone</v-icon>
          <v-icon small v-else-if="props.item.device.type === 'tv'">mdi-television</v-icon>
          <v-icon small v-else>mdi-cellphone-link</v-icon>
          &nbsp;
        </span>
          <span v-if="props.item.device.browser">
            <b>{{ props.item.device.browser.name }}</b> on
          </span>
          <span v-if="props.item.device.os">{{ props.item.device.os.name }}
            {{ props.item.device.os.version }}
          </span>
        </div>
      </td>
      <td class="text-xs-center">{{ props.item.created_at | date }}</td>
      <td class="text-xs-center">
        <div v-if="props.item.location
        && props.item.location.city_name && props.item.location.country_name">
          {{ props.item.location.city_name }}, {{ props.item.location.country_name }}
        </div>
      </td>
      <td class="text-xs-center">{{ props.item.ip }}</td>
      <td class="justify-center layout px-0">
        <v-btn
        outline
        small
        color="success"
        :ripple="false"
        v-if="props.item.id === current.id">
        current
      </v-btn>
      <v-btn
      v-else
      small
      color="error"
      @click="revoke_session(props.item.id)"
      :loading="props.item.is_loading">
      revoke
    </v-btn>
  </td>
</template>
</v-data-table>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
const { log } = require('@bloom42/astro');
import api from '@/bloom/kernel/api';


@Component
export default class SessionList extends Vue {
  @Prop({ type: Array, default: [] })  sessions!: any[];
  @Prop({ type: Object, default: null })  current!: object | null;

  headers = [
    {
      align: 'center',
      sortable: false,
      text: 'Device',
      value: 'device',
    },
    {
      align: 'center',
      sortable: false,
      text: 'Signed In',
      value: 'created_at',
    },
    {
      align: 'center',
      sortable: false,
      text: 'Location',
      value: 'location',
    },
    {
      align: 'center',
      sortable: false,
      text: 'IP',
      value: 'ip',
    },
    {
      align: 'center',
      sortable: false,
      text: 'Actions',
      value: 'id',
    },
  ];


  async revoke_session(id: string) {
    try {
      await api.post(`${api.MYACCOUNT}/v1/me/sessions/${id}/revoke`);
      this.$emit('update', id);
    } catch (err) {
      log.error(err);
    }
  }
}
</script>
