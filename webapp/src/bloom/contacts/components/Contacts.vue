<template>
<div>
  <v-toolbar flat dense color="white" class="hidden-sm-and-down">
    <v-spacer></v-spacer>
    <v-btn outline color="primary" class="add-btn" @click="open_dialog">
      <v-icon left>mdi-plus</v-icon>Create Contact
    </v-btn>
    <!-- <v-toolbar-items>
      <v-tooltip bottom>
        <v-btn slot="activator" icon :disabled="selected.length === 0" @click="delete_contacts">
          <v-icon color="grey darken-1">mdi-delete</v-icon>
        </v-btn>
        <span>Delete</span>
      </v-tooltip>
    </v-toolbar-items> -->
  </v-toolbar>


  <v-data-table
  :headers="headers"
  :items="contacts"
  item-key="id"
  hide-actions
  :loading="is_loading"
  v-model="selected">
    <template slot="no-data">
      <p class="text-xs-center">
        No Contacts.
      </p>
    </template>
    <template slot="items" slot-scope="props">
      <tr class="pointer"
      :active="props.selected"
      @click="current_contact = props.item; open_dialog()">

        <td class="text-xs-left">
          <span>{{ props.item.first_name }} {{ props.item.last_name}}</span>
        </td>
        <td class="text-xs-left">
          <span v-if="props.item.emails.length >= 1">
            {{ props.item.emails[0].email }}
          </span>
        </td>
        <td class="text-xs-left">
          <span v-if="props.item.phones.length >= 1">
            {{ props.item.phones[0].phone }}
          </span>
        </td>
        <td class="text-xs-left">
          <span v-if="props.item.organizations.length >= 1">
            <span v-if="props.item.organizations[0].title !== null">
              {{ props.item.organizations[0].title }},
            </span>
            <span v-if="props.item.organizations[0].name !== null">
              {{ props.item.organizations[0].name }}
            </span>
          </span>
        </td>
       </tr>
    </template>
  </v-data-table>


  <v-btn
    @click="open_dialog" color="red" dark
    fab fixed bottom right class="hidden-md-and-up"
  >
    <v-icon>mdi-plus</v-icon>
  </v-btn>


  <blm-contacts-contact
    :contact="current_contact"
    :visible="contact_dialog"
    @close="close_contact_dialog"
    @create="contact_created"
    @update="contact_updated"
    @delete="contact_deleted"
  />
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import Contact from './Contact.vue';


@Component({
  components: {
    'blm-contacts-contact': Contact,
  },
})
export default class Contacts extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  contact_dialog = false;
  selected: any[] = [];
  contacts: any[] = [];
  headers = [
    {
      align: 'left',
      sortable: false,
      text: 'Name',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Email',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Phone number',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Job title & company',
    },
  ];
  current_contact: any | null = null;


  // computed
  get search_query() {
    return this.$store.state.search;
  }


  // lifecycle
  created() {
    this.fetch_data();
  }


  // watch
  @Watch('search_query')
  on_search_query_change() {
    if (this.search_query) {
      this.search(this.search_query);
    }
  }

  // methods
  async fetch_data() {
    this.error = '';
    this.is_loading = true;
    try {
      this.contacts = await api.get(`${api.CONTACTS}/v1/contacts`);
    } catch (err) {
      // this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }


  async delete_contacts() {
    this.error = '';
    try {
      const contacts_ids = this.selected.map((f: any) => f.id);
      this.is_loading = true;
      await Promise.all(contacts_ids.map((contact_id) => {
        return api.delete(`${api.CONTACTS}/v1/contacts/${contact_id}`);
      }));
      this.contacts = this.contacts.filter((f: any) => contacts_ids.indexOf(f.id) === -1);
      this.selected = [];
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  open_dialog() {
    this.contact_dialog = true;
  }

  close_contact_dialog() {
    this.contact_dialog = false;
    this.current_contact = null;
  }

  contact_created(contact: any) {
    this.contacts.push(contact);
  }

  contact_updated(updated_contact: any) {
    this.contacts = this.contacts.map((contact: any) => {
      return contact.id === updated_contact.id ? updated_contact : contact;
    });
  }

  contact_deleted(deleted_contact: any) {
    this.contacts = this.contacts.filter((c) => c.id !== deleted_contact.id);
  }

  search(query: string) {
    const _ = query; // TODO
  }
}
</script>


<style scoped lang="scss">
</style>
