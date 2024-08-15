
// AheadLibEx.h: PROJECT_NAME 应用程序的主头文件
//

#pragma once

#ifndef __AFXWIN_H__
	#error "在包含此文件之前包含 'pch.h' 以生成 PCH"
#endif

#include "../res/resource.h"
class ahead_lib_ex_app final :  public CWinApp
{
public:
	ahead_lib_ex_app();
    BOOL InitInstance() override;
	DECLARE_MESSAGE_MAP()
};

extern ahead_lib_ex_app the_app;
