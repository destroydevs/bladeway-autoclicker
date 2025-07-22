/*
    BladeWay Autoclicker Program
    Copyright (C) 2025  Evgeny K.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

fn main() {
    let _ = winres::WindowsResource::new()
        .set_icon("icon.ico")
        .set("FileDescription", "Автокликер BladeWay")
        .set("ProductName", "BladeWay Autoclicker")
        .set("LegalCopyright", "Copyright © 2025 BladeWay")
        .set("LegalTrademark", "BladeWay")
        .compile();
}
