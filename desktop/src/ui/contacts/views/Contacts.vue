<template>
  <div>
    <v-container fluid>
      <v-toolbar flat dense>
        <v-spacer />
        <v-btn color="primary" @click="currentContact = null; openContactDialog()">
          <v-icon left>mdi-plus</v-icon>Create Contact
        </v-btn>
      </v-toolbar>

      <v-data-table
        :headers="headers"
        :items="contacts"
        item-key="id"
        hide-default-footer
        :loading="isLoading"
      >
        <template v-slot:no-data>
          <p class="text-center">
            No Contacts.
          </p>
        </template>
       <template v-slot:item="{ item }">
          <tr
            class="blm-pointer"
            @click="currentContact = item; openContactDialog()"
          >

            <td class="text-left">
              <span>{{ item.first_name }} {{ item.last_name}}</span>
            </td>
            <td class="text-left">
              <span v-if="item.emails.length >= 1">
                {{ item.emails[0].email }}
              </span>
            </td>
            <td class="text-left">
              <span v-if="item.phones.length >= 1">
                {{ item.phones[0].phone }}
              </span>
            </td>
            <td class="text-left">
              <span v-if="item.organizations.length >= 1">
                <span v-if="item.organizations[0].title !== ''">
                  {{ item.organizations[0].title }},
                </span>
                <span v-if="item.organizations[0].name !== ''">
                  {{ item.organizations[0].name }}
                </span>
              </span>
            </td>
          </tr>
        </template>
      </v-data-table>

    </v-container>

  <blm-contacts-dialog-contact
    :contact="currentContact"
    :visible="showContactDialog"
    @closed="contactDialogClosed"
    @created="contactCreated"
    @updated="contactUpdated"
    @deleted="contactDeleted"
  />
  </div>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import core from '@/core';
import { Contact, Contacts, Method } from '@/core/contacts';
import ContactDialog from '../components/ContactDialog.vue';


const { log } = require('@bloom42/astro');

@Component({
  components: {
    'blm-contacts-dialog-contact': ContactDialog,
  },
})
export default class Index extends Vue {
  // props
  // data
  error = '';
  isLoading = false;
  contacts: Contact[] = [];
  showContactDialog = false;
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
  currentContact: Contact | null = null;

  // computed
  // lifecycle
  async created() {
    this.findContacts();
  }

  // watch
  // methods
  async findContacts() {
    this.error = '';
    this.isLoading = true;
    try {
      const res = await core.call(Method.ListContacts, core.Empty);
      this.contacts = (res as Contacts).contacts;
    } catch (err) {
      log.error(err);
    } finally {
      this.isLoading = false;
    }
  }

  openContactDialog() {
    this.showContactDialog = true;
  }

  contactDialogClosed() {
    this.showContactDialog = false;
  }

  contactCreated(contact: Contact) {
    this.contacts = [contact, ...this.contacts];
  }

  contactUpdated(updatedContact: Contact) {
    this.contacts = this.contacts.map((note: any) => {
      if (note.id === updatedContact.id) {
        return updatedContact;
      }
      return note;
    });
  }

  contactDeleted(deletedContact: Contact) {
    this.contacts = this.contacts.filter(
      (contact: Contact) => contact.id !== deletedContact.id,
    );
    this.currentContact = null;
  }
}
</script>


<style lang="scss" scoped>
</style>
