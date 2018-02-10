/*
 * Copyright 2014-2018 Miguel Peláez <kernelfreeze@outlook.com>
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software
 * and associated documentation files (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge, publish, distribute,
 * sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all copies or
 * substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
 * BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

import accessors;
import configuration;

/// Util class with some Game data
final class Litecraft {
    @Read private static Litecraft _instance;

    @Read @Write private ConfigurationAdapter _configuration;

    @Read private static const string _litecraft = "A1";
    @Read private static const string _minecraft = "1.13";
    @Read private static const string _clientbrand = "vanilla/litecraft";

    @Read @Write private static string _opengl;
    @Read @Write private static string _glVendor;

    /// Create a new instance of Litecraft main class
    this(ConfigurationAdapter configuration) {
        _configuration = configuration;
        _instance = this;
    }

    /// Get client width
    auto width() {
        return _configuration.width;
    }

    /// Get client height
    auto height() {
        return _configuration.height;
    }

    mixin(GenerateFieldAccessors);
}
