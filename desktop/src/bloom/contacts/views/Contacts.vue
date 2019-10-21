<template>
  <div>
    <v-container fluid>
      <v-toolbar
        flat
        dense
      >
        <v-spacer />
        <v-btn
          outlined
          color="primary"
          class="add-btn"
          @click="currentContact = null; openContactDialog()"
        >
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
        <template slot="no-data">
          <p class="text-xs-center">
            No Contacts.
          </p>
        </template>
        <template
          slot="items"
          slot-scope="props"
        >
          <tr
            class="blm-pointer"
            @click="currentContact = props.item; openContactDialog()"
          >

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
import { Component, Prop, Vue } from 'vue-property-decorator';
import { Native, Message } from '@/native';
import { Contact, GuiContacts } from '@/native/messages/contacts';
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
    const message: Message = {
      type: 'contacts.gui.list_contacts',
      data: {},
    };
    try {
      const res = await Native.call(message);
      this.contacts = (res.data as GuiContacts).contacts;
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
    const pos = this.contacts
      .map((contact: Contact) => contact.id)
      .indexOf(updatedContact.id);
    this.contacts.splice(pos, 1);
    this.contacts = [updatedContact, ...this.contacts];
  }

  contactDeleted(deletedContact: Contact) {
    this.contacts = this.contacts.filter(
      (contact: Contact) => contact.id !== deletedContact.id,
    );
  }
}
</script>


<style lang="scss" scoped>
</style>
