<template>
  <v-container>
    <v-row justify="center">
      <v-col cols="12">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>


    <v-row class="text-center" v-if="user">


      <v-col cols="12">
        <v-avatar color="white" size="128">
          <v-img :src="user.avatarUrl" v-if="user.avatarUrl"/>
          <v-icon medium color="grey" v-else>mdi-account</v-icon>
        </v-avatar>
      </v-col>

      <v-col cols="12">
        <h2 class="headline">@{{ user.username }}</h2>
        <span class="subheader">{{ user.id }}</span>
      </v-col>
    </v-row>

    <v-row class="text-left" v-if="user">

      <v-col cols="12">
        <h1>Info</h1>
      </v-col>

      <v-col cols="6">
        <v-text-field label="Display name" disabled :value="user.displayName" />
      </v-col>

      <v-col cols="6">
        <v-text-field label="Email" disabled :value="user.email" />
      </v-col>

      <v-col cols="6">
        <v-text-field label="First name" disabled :value="user.firstName" />
      </v-col>
      <v-col cols="6">
        <v-text-field label="Last name" disabled :value="user.lastName" />
      </v-col>

      <v-col cols="12">
        <v-textarea disabled label="Bio" :vlaue="user.bio" />
      </v-col>

      <v-col cols="12">
        <h1>Billing</h1>
      </v-col>

      <v-col cols="12">
        <h2 class="headline">Invoices</h2>
      </v-col>
      <v-col cols="12">
        <blm-billing-table-invoices :loading="loading" :invoices="invoices" />
      </v-col>

      <v-col cols="12">
        <h2 class="headline">Payment methods</h2>
      </v-col>
      <v-col cols="12">
        <blm-billing-table-payment-methods :loading="loading" :payment-methods="paymentMethods" />
      </v-col>

      <v-col cols="12">
        <h1>Groups</h1>
        <blm-groups-simple-table :loading="loading" :groups="groups" inspect-url="/admin/groups" />
      </v-col>


      <v-col cols="12">
        <h1>Actions</h1>
      </v-col>

      <v-col cols="12">
        <v-btn color="success" @click="enableUser" v-if="user.disabledAt">Enable User</v-btn>
        <v-btn color="error" @click="disableUser" v-else>Disable User</v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import {
  User, Invoice, PaymentMethod, InvoiceEdge, Maybe, PaymentMethodEdge, Group, GroupEdge,
} from '@/api/models';
import {
  FetchUserParams, Method, EnableUserParams, DisableUserParams,
} from '@/core/users';
import core from '@/core';
import PaymentMethodsTable from '@/ui/billing/components/payment_methods_table.vue';
import InvoicesTable from '@/ui/billing/components/invoices_table.vue';
import BlmGroupsSimpleTable from '@/ui/groups/components/simple_table.vue';


@Component({
  components: {
    'blm-billing-table-payment-methods': PaymentMethodsTable,
    'blm-billing-table-invoices': InvoicesTable,
    BlmGroupsSimpleTable,
  },
})
export default class AdminUserView extends Vue {
  // props
  // data
  error = '';
  user: User | null = null;
  loading = false;
  username = '';

  // computed
  get invoices(): Invoice[] {
    if (this.user === null) {
      return [];
    }
    return this.user.invoices!.edges!.map((edge: Maybe<InvoiceEdge>) => edge!.node!);
  }

  get paymentMethods(): PaymentMethod[] {
    if (this.user === null) {
      return [];
    }
    return this.user.paymentMethods!
      .edges!.map((edge: Maybe<PaymentMethodEdge>) => edge!.node!);
  }

  get groups(): Group[] {
    if (this.user === null) {
      return [];
    }
    return this.user.groups!
      .edges!.map((edge: Maybe<GroupEdge>) => edge!.node!);
  }

  // lifecycle
  created() {
    this.username = this.$route.params.username;
    this.fetchData();
  }

  // watch
  // methods
  async fetchData() {
    this.error = '';
    this.loading = true;
    const params: FetchUserParams = {
      username: this.username,
    };

    try {
      this.user = await core.call(Method.FetchUserDetails, params);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  async enableUser() {
    this.error = '';
    this.loading = true;
    const params: EnableUserParams = {
      id: this.user!.id!,
    };

    try {
      this.user = await core.call(Method.EnableUser, params);
      this.user!.disabledAt = null;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  async disableUser() {
    this.error = '';
    this.loading = true;
    const params: DisableUserParams = {
      id: this.user!.id!,
    };

    try {
      this.user = await core.call(Method.DisableUser, params);
      this.user!.disabledAt = new Date();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }
}
</script>


<style lang="scss" scoped>
</style>
