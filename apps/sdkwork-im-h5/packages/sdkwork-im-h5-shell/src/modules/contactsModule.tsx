import React from "react";
import { Users } from "lucide-react";

import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ContactsComponentName = "AddFriend" | "AddressBook" | "NewFriends" | "OrganizationList";

function lazyContactsComponent(name: ContactsComponentName) {
  return React.lazy(async () => {
    const contactsModule = await import("@sdkwork/im-h5-contacts");
    return { default: contactsModule[name] };
  });
}

const AddFriend = lazyContactsComponent("AddFriend");
const AddressBook = lazyContactsComponent("AddressBook");
const NewFriends = lazyContactsComponent("NewFriends");
const OrganizationList = lazyContactsComponent("OrganizationList");

export const contactsModule: ImH5CapabilityModule = {
  id: "contacts",
  navigation: [
    {
      id: "contacts",
      moduleId: "contacts",
      path: "/workspace/contacts",
      labelKey: "contacts.title",
      icon: Users,
    },
  ],
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.contactsIndex, render: () => <AddressBook /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsSearch, render: () => <AddFriend /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsAdd, render: () => <AddFriend /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsFriendRequests, render: () => <NewFriends /> },
    {
      ...IM_H5_ROUTE_DEFINITIONS.contactsOrganization,
      render: () => <OrganizationList />,
    },
  ],
};
