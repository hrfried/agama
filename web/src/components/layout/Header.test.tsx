/*
 * Copyright (c) [2024-2025] SUSE LLC
 *
 * All Rights Reserved.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, contact SUSE LLC.
 *
 * To contact SUSE LLC about this file by physical or electronic mail, you may
 * find current contact information at www.suse.com.
 */

import React from "react";
import { screen, within } from "@testing-library/react";
import { plainRender, installerRender } from "~/test-utils";
import { Product } from "~/types/software";
import Header from "./Header";

const tumbleweed: Product = {
  id: "Tumbleweed",
  name: "openSUSE Tumbleweed",
  description: "Tumbleweed description...",
  registration: false,
};

const microos: Product = {
  id: "MicroOS",
  name: "openSUSE MicroOS",
  description: "MicroOS description",
  registration: false,
};

jest.mock("~/components/core/InstallerOptions", () => () => <div>Installer Options Mock</div>);
jest.mock("~/components/core/InstallButton", () => () => <div>Install Button Mock</div>);

jest.mock("~/queries/software", () => ({
  useProduct: () => ({
    products: [tumbleweed, microos],
    selectedProduct: tumbleweed,
  }),
  useRegistration: () => undefined,
}));

describe("Header", () => {
  it("renders the product name unless showProductName is set to false", () => {
    const { rerender } = plainRender(<Header />);
    screen.getByRole("heading", { name: tumbleweed.name, level: 1 });
    rerender(<Header />);
    screen.getByRole("heading", { name: tumbleweed.name, level: 1 });
    rerender(<Header showProductName={false} />);
    expect(screen.queryByRole("heading", { name: tumbleweed.name, level: 1 })).toBeNull();
  });

  it("mounts the Install button", () => {
    plainRender(<Header />);
    screen.getByText("Install Button Mock");
  });

  it("mounts InstallerOptions", () => {
    plainRender(<Header />);
    screen.getByText("Installer Options Mock");
  });

  it("renders skip to content link", async () => {
    plainRender(<Header />);
    screen.getByRole("link", { name: "Skip to content" });
  });

  it("does not render skip to content link when showSkipToContent is false", async () => {
    plainRender(<Header showSkipToContent={false} />);
    expect(screen.queryByRole("link", { name: "Skip to content" })).toBeNull();
  });

  it("renders an options dropdown by default", async () => {
    const { user } = installerRender(<Header />);
    expect(screen.queryByRole("menu")).toBeNull();
    const toggler = screen.getByRole("button", { name: "Options toggle" });
    await user.click(toggler);
    const menu = screen.getByRole("menu");
    within(menu).getByRole("menuitem", { name: "Change product" });
    within(menu).getByRole("menuitem", { name: "Download logs" });
  });

  it("does not render an options dropdown when showInstallerOptions is false", async () => {
    installerRender(<Header showInstallerOptions={false} />);
    expect(screen.queryByRole("button", { name: "Options toggle" })).toBeNull();
  });

  it.todo("allows downloading the logs");
});
