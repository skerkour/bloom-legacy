<template>
  <v-container fluid>
    <v-row>
      <v-col cols="3" class="blm-scrollable">
        <v-list two-line avatar>
        <v-list-item-group v-model="selectedConversation" color="primary">
          <v-list-item v-for="(conversation, i) in conversations" :key="i">
            <v-list-item-avatar>
              <v-img :src="conversation.avatar"></v-img>
            </v-list-item-avatar>
            <v-list-item-content class="text-left">
              <v-list-item-title>{{ conversation.title}} </v-list-item-title>
              <v-list-item-subtitle>{{ conversation.subtitle }}</v-list-item-subtitle>
            </v-list-item-content>
          </v-list-item>
        </v-list-item-group>
      </v-list>

      </v-col>

      <v-col cols="9" class="blm-chat-container">
        <v-toolbar flat id="blm-chat-toolbar" dense>
          <v-avatar size="42">
            <v-img :src="conversations[selectedConversation].avatar" />
          </v-avatar>
          <v-toolbar-title id="blm-chat-toolbar-title">
            {{ conversations[selectedConversation].title }}
          </v-toolbar-title>
          <v-spacer />

          <v-btn icon>
            <v-icon>mdi-dots-vertical</v-icon>
          </v-btn>
        </v-toolbar>


        <v-sheet
          id="blm-chat-messages"
          class="overflow-y-auto"
        >
          <v-container>
            <v-list three-line>
              <template v-for="(item, index) in messages">
                <v-divider
                  v-if="item.divider"
                  :key="index"
                  :inset="item.inset"
                ></v-divider>

                <v-list-item
                  v-else
                  :key="item.title"
                  class="text-left"
                >
                  <v-list-item-avatar>
                    <v-img :src="item.avatar"></v-img>
                  </v-list-item-avatar>

                  <v-list-item-content>
                    <v-list-item-subtitle>{{ item.subtitle }}</v-list-item-subtitle>
                  </v-list-item-content>
                </v-list-item>
              </template>
            </v-list>
          </v-container>
        </v-sheet>


        <div id="blm-chat-input">
          <v-textarea
            outlined
            placeholder="Send a message"
            rows="3"
          />
        </div>


      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';


@Component
export default class Index extends Vue {
  // props
  // data
  selectedConversation = 0;
  conversations = [
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/1.jpg',
      title: 'Brunch this weekend?',
      subtitle: 'I\'ll be in your neighborhood doing errands this weekend. Do you want to hang out?',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/2.jpg',
      title: 'Summer BBQ',
      subtitle: 'Wish I could come, but I\'m out of town this weekend.',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/3.jpg',
      title: 'Oui oui',
      subtitle: 'Do you have Paris recommendations? Have you ever been?',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/4.jpg',
      title: 'Birthday gift',
      subtitle: 'Have any ideas about what we should get Heidi for her birthday?',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: 'We should eat this: Grate, Squash, Corn, and tomatillo Tacos.',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: 'We should eat this: Grate, Squash, Corn, and tomatillo Tacos.',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: 'We should eat this: Grate, Squash, Corn, and tomatillo Tacos.',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: 'We should eat this: Grate, Squash, Corn, and tomatillo Tacos.',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: 'We should eat this: Grate, Squash, Corn, and tomatillo Tacos.',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: 'We should eat this: Grate, Squash, Corn, and tomatillo Tacos.',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: 'We should eat this: Grate, Squash, Corn, and tomatillo Tacos.',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: 'We should eat this: Grate, Squash, Corn, and tomatillo Tacos.',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: 'We should eat this: Grate, Squash, Corn, and tomatillo Tacos.',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: 'We should eat this: Grate, Squash, Corn, and tomatillo Tacos.',
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: 'We should eat this: Grate, Squash, Corn, and tomatillo Tacos.',
    },

  ];

  messages = [
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/1.jpg',
      title: 'Brunch this weekend?',
      subtitle: "<span class='text--primary'>Ali Connors</span> &mdash; I'll be in your neighborhood doing errands this weekend. Do you want to hang out?",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/2.jpg',
      title: 'Summer BBQ <span class="grey--text text--lighten-1">4</span>',
      subtitle: "<span class='text--primary'>to Alex, Scott, Jennifer</span> &mdash; Wish I could come, but I'm out of town this weekend.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/3.jpg',
      title: 'Oui oui',
      subtitle: "<span class='text--primary'>Sandra Adams</span> &mdash; Do you have Paris recommendations? Have you ever been?",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/4.jpg',
      title: 'Birthday gift',
      subtitle: "<span class='text--primary'>Trevor Hansen</span> &mdash; Have any ideas about what we should get Heidi for her birthday?",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    { divider: true, inset: true },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
  ];
  // computed
  // lifecycle
  // watch
  // methods
  lol() {
  }
}

// import { Component, Prop, Vue } from 'vue-property-decorator';
// import { Subscription } from 'rxjs';
// import Native from '@/native';

// @Component
// export default class Index extends Vue {
//   // props
//   // data
//   subscription: Subscription | null = null;
//   result = '';
//   count = 0;

//   // computed
//   // lifecycle
//   async created() {
//     this.subscription = Native.notification.subscribe((event: any) => {
//       console.log(event);
//       this.count = event.data.count;
//     });
//     this.result = (await Native.call()).count;
//   }

//   beforeDestroy() {
//     if (this.subscription) {
//       this.subscription.unsubscribe();
//     }
//   }

//   // watch
//   // methods
// }
</script>


<style lang="scss" scoped>
.blm-scrollable {
  overflow-y: auto;
  height: calc(100vh - 90px);
}

.blm-chat-container{
  // height: calc(100vh - 9.5rem);
  overflow-y: auto;
}

#blm-chat-toolbar {
  border-bottom: 1px solid rgba($color: #000000, $alpha: 0.1);
}

#blm-chat-toolbar-title {
  margin-left: 10px;
}

#blm-chat-input {
  position: absolute;
  bottom: 0;
  width: calc(100vw - 90px - 25vw);
  height: 12vh;
}

#blm-chat-messages {
  height: calc(100vh - 156px - 12vh); // 156 is all toolbars + padding
}
</style>
