<template>
  <v-dialog
    v-model="show"
    @keydown.esc="close()"
    @click:outside="close()"
    scrollable
    width="50%"
    :fullscreen="$vuetify.breakpoint.smAndDown"
  >
    <v-card>
      <v-card-title
        class="headline"
        v-if="contact === null"
      >
        <h3 class="headline mb-0">Create new contact</h3>
      </v-card-title>
      <v-card-title
        dark
        class="headline"
        v-else
      >
        <h3 class="headline mb-0">
          <h3 class="headline mb-0">{{ contact.first_name }} {{ contact.last_name }}</h3>
        </h3>
        <v-spacer />
        <v-tooltip bottom>
          <template v-slot:activator="{ on: tooltip }">
            <v-menu slot="activator">
              <template v-slot:activator="{ on: menu }">
                <v-btn
                  slot="activator"
                  text
                  color="white"
                  icon
                  v-on="{ ...tooltip, ...menu }"
                >
                  <v-icon>mdi-dots-vertical</v-icon>
                </v-btn>
              </template>

              <v-list>
                <v-list-item @click="deleteContact">
                  <v-list-item-title>Delete contact</v-list-item-title>
                </v-list-item>
              </v-list>
            </v-menu>
          </template>
          <span>More actions</span>
        </v-tooltip>
      </v-card-title>

      <v-divider />
      <v-card-text>
        <v-alert
          icon="mdi-alert-circle"
          :value="error !== ''"
          type="error"
          dismissible
        >
          {{ error }}
        </v-alert>
        <v-container
          fluid
          grid-list-lg
        >

          <!-- name -->
          <v-row>
            <v-col cols="6">
              <v-text-field
                label="First name"
                v-model="firstName"
                prepend-icon="mdi-account"
              ></v-text-field>
            </v-col>
            <v-col cols="6">
              <v-text-field
                label="Last name"
                v-model="lastName"
              ></v-text-field>
            </v-col>
          </v-row>
          <!-- /name -->

          <!-- birthday -->
          <v-row>
            <v-flex xs12>
              <v-menu
                ref="birthdayMenu"
                :close-on-content-click="false"
                v-model="birthdayMenu"
                :nudge-right="40"
                transition="scale-transition"
                offset-y
                min-width="290px"
              >
                <template v-slot:activator="{ on }">
                  <v-text-field
                    v-on="on"
                    v-model="birthdayFormatted"
                    label="Birthday"
                    prepend-icon="mdi-calendar"
                    @blur="birthday = parseDate(birthdayFormatted)"
                    readonly
                  />
                </template>
                <v-date-picker
                  ref="birthdayPicker"
                  v-model="birthday"
                  :max="new Date().toISOString().substr(0, 10)"
                  min="1900-01-01"
                  @change="saveBirthday"
                ></v-date-picker>
              </v-menu>
            </v-flex>
          </v-row>
          <!-- /birthday -->

          <!-- addresses: TODO(z0mbie42) -->

          <!-- organizations -->
          <v-row>
            <v-col cols="6">
              <v-text-field
                label="Company"
                v-model="organizations[0].name"
                prepend-icon="mdi-domain"
              ></v-text-field>
            </v-col>
            <v-col cols="6">
              <v-text-field
                label="Job title"
                v-model="organizations[0].title"
              ></v-text-field>
            </v-col>
          </v-row>
          <!-- /organizations -->

          <v-row>
            <v-col cols="12">
              <v-list class="pt-0 pb-0">
                <v-list-item
                  v-for="(phone, index) in phones"
                  :key="index"
                  class="contacts-add-row"
                >
                  <v-row align="center">
                    <v-col
                      cols="1"
                      class="pl-0 pr-0"
                    >
                      <v-icon v-if="index === 0">mdi-phone</v-icon>
                    </v-col>
                    <v-col cols="5">
                      <v-text-field
                        label="Phone"
                        v-model="phone.phone"
                      ></v-text-field>
                    </v-col>
                    <v-col cols="4">
                      <v-autocomplete
                        :items="phoneLabels"
                        v-model="phone.label"
                        label="Label"
                        single-line
                      >
                      </v-autocomplete>
                    </v-col>
                    <v-col
                      cols="1"
                      class="pr-0 pl-0"
                    >
                      <v-tooltip bottom>
                        <template v-slot:activator="{ on }">
                          <v-btn
                            text
                            icon
                            small
                            v-on="on"
                            color="grey"
                            class="action-button"
                            @click="removePhone(index)"
                          >
                            <v-icon>mdi-close</v-icon>
                          </v-btn>
                        </template>
                        <span>Remove</span>
                      </v-tooltip>
                    </v-col>
                    <v-col
                      cols="1"
                      class="pl-0 pr-0"
                      v-if="index === phones.length - 1"
                    >
                      <v-tooltip bottom>
                        <template v-slot:activator="{ on }">
                          <v-btn
                            text
                            icon
                            small
                            v-on="on"
                            color="blue darken-2"
                            class="ml-0"
                            @click="addPhone"
                          >
                            <v-icon>mdi-plus-circle</v-icon>
                          </v-btn>
                        </template>
                        <span>Add</span>
                      </v-tooltip>
                    </v-col>
                  </v-row>
                </v-list-item>
              </v-list>
            </v-col>
          </v-row>
          <!-- /phones -->

          <!-- emails -->
          <v-row>
            <v-col cols="12">
              <v-list class="pt-0 pb-0">
                <v-list-item
                  v-for="(email, index) in emails"
                  :key="index"
                  class="contacts-add-row"
                >
                  <v-row align="center">
                    <v-col
                      cols="1"
                      class="pl-0 pr-0"
                    >
                      <v-icon v-if="index === 0">mdi-email-outline</v-icon>
                    </v-col>
                    <v-col cols="5">
                      <v-text-field
                        label="Email"
                        v-model="email.email"
                      ></v-text-field>
                    </v-col>
                    <v-col cols="4">
                      <v-autocomplete
                        :items="emailLabels"
                        v-model="email.label"
                        label="Label"
                        single-line
                      >
                      </v-autocomplete>
                    </v-col>
                    <v-col
                      cols="1"
                      class="pr-0 pl-0"
                    >
                      <v-tooltip bottom>
                        <template v-slot:activator="{ on }">
                          <v-btn
                            text
                            icon
                            small
                            v-on="on"
                            color="grey"
                            class="action-button"
                            @click="removeEmail(index)"
                          >
                            <v-icon>mdi-close</v-icon>
                          </v-btn>
                        </template>
                        <span>Remove</span>
                      </v-tooltip>
                    </v-col>
                    <v-col
                      cols="1"
                      class="pl-0 pr-0"
                    >
                      <v-tooltip
                        bottom
                        v-if="index === emails.length - 1"
                      >
                        <template v-slot:activator="{ on }">
                          <v-btn
                            text
                            icon
                            small
                            v-on="on"
                            color="blue darken-2"
                            class="ml-0"
                            @click="addEmail"
                          >
                            <v-icon>mdi-plus-circle</v-icon>
                          </v-btn>
                        </template>
                        <span>Add</span>
                      </v-tooltip>
                    </v-col>
                  </v-row>
                </v-list-item>
              </v-list>
            </v-col>
          </v-row>
          <!-- /emails -->

          <!-- websites -->
          <v-row>
            <v-col cols="12">
              <v-list class="pt-0 pb-0">
                <v-list-item
                  v-for="(website, index) in websites"
                  :key="index"
                  class="contacts-add-row"
                >
                  <v-row align="center">
                    <v-col
                      cols="1"
                      class="pl-0 pr-0"
                    >
                      <v-icon v-if="index === 0">mdi-earth</v-icon>
                    </v-col>
                    <v-col cols="5">
                      <v-text-field
                        label="Website"
                        v-model="website.website"
                      ></v-text-field>
                    </v-col>
                    <v-col cols="4">
                      <v-autocomplete
                        :items="websiteLabels"
                        v-model="website.label"
                        label="Label"
                        single-line
                      >
                      </v-autocomplete>
                    </v-col>
                    <v-col
                      cols="1"
                      class="pr-0 pl-0"
                    >
                      <v-tooltip bottom>
                        <template v-slot:activator="{ on }">
                          <v-btn
                            text
                            icon
                            small
                            v-on="on"
                            color="grey"
                            class="action-button"
                            @click="removeWebsite(index)"
                          >
                            <v-icon>mdi-close</v-icon>
                          </v-btn>
                        </template>
                        <span>Remove</span>
                      </v-tooltip>
                    </v-col>
                    <v-col
                      cols="1"
                      class="pl-0 pr-0"
                    >
                      <v-tooltip
                        bottom
                        v-if="index === websites.length - 1"
                      >
                        <template v-slot:activator="{ on }">
                          <v-btn
                            text
                            icon
                            small
                            v-on="on"
                            color="blue darken-2"
                            class="ml-0"
                            @click="addWebsite"
                          >
                            <v-icon>mdi-plus-circle</v-icon>
                          </v-btn>
                        </template>
                        <span>Add</span>
                      </v-tooltip>
                    </v-col>
                  </v-row>
                </v-list-item>
              </v-list>
            </v-col>
          </v-row>

          <!-- notes -->
          <v-row>
            <v-col cols="12">
              <v-textarea
                label="Notes"
                v-model="notes"
                prepend-icon="mdi-calendar-text"
              ></v-textarea>
            </v-col>
          </v-row>

        </v-container>
      </v-card-text>
    </v-card>
  </v-dialog>
</template>


<script lang="ts">
// TODO(z0mbie42): addresses
import {
  Component,
  Prop,
  Vue,
  Watch,
} from 'vue-property-decorator';
import {
  Contact,
  Organization,
  Website,
  Email,
  Phone,
  GuiContact,
} from '@/native/messages/contacts';
import { Native, Message } from '@/native';

const DEFAULT_EMAIL = { email: '', label: 'Other' };
const DEFAULT_WEBSITE = { website: '', label: 'Other' };
const DEFAULT_PHONE = { phone: '', label: 'Other' };
const DEFAULT_ORGANIZATION = { name: '', title: '' };

@Component
export default class ContactDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;
  @Prop({ type: Object, default: null }) contact!: Contact | null;

  // data
  error = '';
  isLoading = false;
  firstName: string = '';
  lastName: string = '';
  birthday: any | null = null;
  birthdayFormatted: any | null = null;
  birthdayMenu = false;
  notes: string = '';
  organizations: Organization[] = [Object.assign({}, DEFAULT_ORGANIZATION)];
  phones: Phone[] = [Object.assign({}, DEFAULT_PHONE)];
  emails: Email[] = [Object.assign({}, DEFAULT_EMAIL)];
  websites: Website[] = [Object.assign({}, DEFAULT_WEBSITE)];
  phoneLabels = [
    'Home',
    'Work',
    'Other',
    'Mobile',
    'Main',
    'Home fax',
    'Work fax',
  ];
  addressLabels = ['Home', 'Work', 'Other'];
  websiteLabels = ['Profile', 'Blog', 'Home page', 'Work', 'Other'];
  emailLabels = ['Home', 'Work', 'Other'];

  // computed
  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      // this.save();
      this.$emit('closed');
    }
  }

  // lifecycle
  // watch
  @Watch('birthday')
  onBirthdayUpdated(_val: any) {
    this.birthdayFormatted = this.formatDate(this.birthday);
  }

  // methods
  async close() {
    await this.save();
    this.show = false;
  }

  save() {
    if (this.contact) {
      this.updateContact();
    } else {
      this.createContact();
    }
  }

  async createContact() {
    this.error = '';
    this.isLoading = true;
    if (this.isEmpty()) { // isEmpty
      return;
    }
    const message: Message = {
      type: 'contacts.gui.create_contact',
      data: {
        birthday: this.birthday,
        first_name: this.firstName,
        last_name: this.lastName,
        notes: this.notes,
        emails: this.emails,
        phones: this.phones,
        websites: this.websites,
        organizations: this.organizations,
        addresses: [],
      },
    };
    try {
      const res = await Native.call(message);
      this.$emit('created', (res.data as GuiContact).contact);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async updateContact() {
    this.error = '';
    this.isLoading = true;
    const contact = { ...this.contact } as Contact;
    contact.birthday = this.birthday;
    contact.first_name = this.firstName;
    contact.last_name = this.lastName;
    contact.notes = this.notes;
    contact.emails = this.emails;
    contact.phones = this.phones;
    contact.websites = this.websites;
    contact.organizations = this.organizations;
    contact.addresses = [];
    const message: Message = {
      type: 'contacts.gui.update_contact',
      data: {
        contact: this.contact!,
      },
    };
    try {
      const res = await Native.call(message);
      this.$emit('updated', (res.data as GuiContact).contact);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async deleteContact() {
    this.error = '';
    this.isLoading = true;
    const message: Message = {
      type: 'contacts.gui.delete_contact',
      data: {
        id: this.contact!.id,
      },
    };
    try {
      const res = await Native.call(message);
      this.$emit('deleted', this.contact);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
    this.close();
  }

  addPhone() {
    this.phones.push(Object.assign({}, DEFAULT_PHONE));
  }

  removePhone(index: number) {
    this.phones.splice(index, 1);
    if (this.phones.length === 0) {
      this.addPhone();
    }
  }

  addEmail() {
    this.emails.push(Object.assign({}, DEFAULT_EMAIL));
  }

  removeEmail(index: number) {
    this.emails.splice(index, 1);
    if (this.emails.length === 0) {
      this.addEmail();
    }
  }

  addWebsite() {
    this.websites.push(Object.assign({}, DEFAULT_WEBSITE));
  }
  removeWebsite(index: number) {
    this.websites.splice(index, 1);
    if (this.websites.length === 0) {
      this.addWebsite();
    }
  }

  saveBirthday(date: any) {
    (this.$refs.birthdayMenu as any).save(date);
  }

  formatDate(date: any) {
    if (!date) {
      return null;
    }

    const [year, month, day] = date.split('-');
    return `${year}/${month}/${day}`;
  }

  parseDate(date: any) {
    if (!date) {
      return null;
    }

    const [year, month, day] = date.split('/');
    return `${year}-${month.padStart(2, '0')}-${day.padStart(2, '0')}`;
  }

  isEmpty(): boolean {
    return false;
  }
}
</script>


<style lang="scss" scoped>
</style>
