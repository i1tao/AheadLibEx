#pragma once
#include "pch.h"
#include <Windows.h>

#include <ImageHlp.h>
#pragma comment(lib,"Dbghelp.lib")
/***********************************************************************************************/
/*³£¹æ×Ö·û´®*/
#define STR_AUTHOR_NAME			TEXT("i1tao")
#define STR_EMAIL				TEXT("Email:lumosmagicb00m@gmail.com")
#define STR_GITHUB_ADDRESS		TEXT("Github : https://github.com/i1tao/AheadLibEx")
#define STR_BLOG				TEXT("Website : https://www.cnblogs.com/0xc5/")
#define STR_51ASM				TEXT("        : https://www.51asm.com")
#define STR_BUILD_VERSION		TEXT("1.0")
#define STR_BUILD_DATE			TEXT(__DATE__)
#define STR_BUILD_TIME			TEXT(__TIME__)
#define STR_BUILD_DATE_TIME		TEXT(__DATE__) TEXT(" ") TEXT(__TIME__)
#define STR_PROC_TITLE          TEXT("AheadLibEx Ver:") STR_BUILD_VERSION TEXT(" Build:") STR_BUILD_DATE
#define STR_COPYRIGHT			TEXT("Copyright (C) 2023 ") STR_AUTHOR_NAME TEXT(" . All rights reserved.")
/***********************************************************************************************/