/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019-2021 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

#ifndef ESPANSO_X11_CLIPBOARD_H
#define ESPANSO_X11_CLIPBOARD_H

#include <stdint.h>

extern "C" int32_t clipboard_x11_get_text(char * buffer, int32_t buffer_size);

extern "C" int32_t clipboard_x11_set_text(char * text);
extern "C" int32_t clipboard_x11_set_html(char * html, char * fallback_text);
extern "C" int32_t clipboard_x11_set_image(char * buffer, int32_t buffer_size);

#endif //ESPANSO_X11_CLIPBOARD_H