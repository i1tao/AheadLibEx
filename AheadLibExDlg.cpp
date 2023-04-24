
// AheadLibExDlg.cpp: 实现文件
//

#include "pch.h"
#include "framework.h"
#include "AheadLibEx.h"
#include "AheadLibExDlg.h"
#include "afxdialogex.h"

#ifdef _DEBUG
#define new DEBUG_NEW
#endif


// 用于应用程序“关于”菜单项的 CAboutDlg 对话框

class CAboutDlg : public CDialogEx
{
public:
	CAboutDlg();

	// 对话框数据
#ifdef AFX_DESIGN_TIME
	enum { IDD = IDD_ABOUTBOX };
#endif

protected:
	virtual void DoDataExchange(CDataExchange* pDX);    // DDX/DDV 支持

	// 实现
protected:
	DECLARE_MESSAGE_MAP()
};

CAboutDlg::CAboutDlg() : CDialogEx(IDD_ABOUTBOX)
{
}

void CAboutDlg::DoDataExchange(CDataExchange* pDX)
{
	CDialogEx::DoDataExchange(pDX);
}

BEGIN_MESSAGE_MAP(CAboutDlg, CDialogEx)
END_MESSAGE_MAP()


// CAheadLibExDlg 对话框



CAheadLibExDlg::CAheadLibExDlg(CWnd* pParent /*=nullptr*/)
	: CDialogEx(IDD_AHEADLIBEX_DIALOG, pParent)
{
	m_hIcon = AfxGetApp()->LoadIcon(IDR_MAINFRAME);
}

void CAheadLibExDlg::DoDataExchange(CDataExchange* pDX)
{
	CDialogEx::DoDataExchange(pDX);
	DDX_Control(pDX, IDC_EDIT_INPUTFILE, m_editInputFile);
	DDX_Control(pDX, IDC_EDIT_OUTPUTFILE, m_editOutputFile);
	DDX_Control(pDX, IDC_EDIT_OUTPUTINFO, m_editInfo);
	DDX_Control(pDX, IDC_EDIT_OUTPUTPROJECT, m_editOutputProject);
}

BEGIN_MESSAGE_MAP(CAheadLibExDlg, CDialogEx)
	ON_WM_SYSCOMMAND()
	ON_WM_PAINT()
	ON_WM_QUERYDRAGICON()
	ON_BN_CLICKED(IDC_BUTTON_INPUTFILE, &CAheadLibExDlg::OnBnClickedButtonInputfile)
	ON_BN_CLICKED(IDC_BUTTON_OUTPUTFILE, &CAheadLibExDlg::OnBnClickedButtonOutputfile)
	ON_WM_DROPFILES()
	ON_BN_CLICKED(IDC_RADIO_CPP, &CAheadLibExDlg::OnBnClickedRadioCpp)
	ON_BN_CLICKED(IDC_BUTTON_OUTPUTPROJECT, &CAheadLibExDlg::OnBnClickedButtonOutputProject)
	ON_BN_CLICKED(IDC_RADIO_PROJECT, &CAheadLibExDlg::OnBnClickedRadioProject)
END_MESSAGE_MAP()


// CAheadLibExDlg 消息处理程序

BOOL CAheadLibExDlg::OnInitDialog()
{
	CDialogEx::OnInitDialog();

	// 将“关于...”菜单项添加到系统菜单中。

	// IDM_ABOUTBOX 必须在系统命令范围内。
	ASSERT((IDM_ABOUTBOX & 0xFFF0) == IDM_ABOUTBOX);
	ASSERT(IDM_ABOUTBOX < 0xF000);

	CMenu* pSysMenu = GetSystemMenu(FALSE);
	if (pSysMenu != nullptr)
	{
		BOOL bNameValid;
		CString strAboutMenu;
		bNameValid = strAboutMenu.LoadString(IDS_ABOUTBOX);
		ASSERT(bNameValid);
		if (!strAboutMenu.IsEmpty())
		{
			pSysMenu->AppendMenu(MF_SEPARATOR);
			pSysMenu->AppendMenu(MF_STRING, IDM_ABOUTBOX, strAboutMenu);
		}
	}

	// 设置此对话框的图标。  当应用程序主窗口不是对话框时，框架将自动
	//  执行此操作
	SetIcon(m_hIcon, TRUE);			// 设置大图标
	SetIcon(m_hIcon, FALSE);		// 设置小图标

	// TODO: 在此添加额外的初始化代码

	//初始化标题
	SetWindowText(STR_PROC_TITLE);

	//初始化Radio控件
	((CButton*)GetDlgItem(IDC_RADIO_CPP))->SetCheck(true);
	m_editOutputProject.EnableWindow(false);
	GetDlgItem(IDC_BUTTON_OUTPUTPROJECT)->EnableWindow(false);

	//初始化欢迎信息

	CString strWelcome;

	strWelcome += _T("Author: " TEXT(" ") STR_AUTHOR_NAME "\r\n");
	strWelcome += (STR_GITHUB_ADDRESS _T("\r\n"));
	strWelcome += STR_COPYRIGHT;
	strWelcome += _T("\r\n");
	strWelcome += _T("------------------------------------------------------------------------\r\n");
	strWelcome += _T("Based on xjun's AheadLib-x86-x64(https://github.com/strivexjun/AheadLib-x86-x64) \r\n");
	m_editInfo.SetWindowText(strWelcome);
	return TRUE;  // 除非将焦点设置到控件，否则返回 TRUE
}

void CAheadLibExDlg::OnSysCommand(UINT nID, LPARAM lParam)
{
	if ((nID & 0xFFF0) == IDM_ABOUTBOX)
	{
		CAboutDlg dlgAbout;
		dlgAbout.DoModal();
	}
	else
	{
		CDialogEx::OnSysCommand(nID, lParam);
	}
}

// 如果向对话框添加最小化按钮，则需要下面的代码
//  来绘制该图标。  对于使用文档/视图模型的 MFC 应用程序，
//  这将由框架自动完成。

void CAheadLibExDlg::OnPaint()
{
	if (IsIconic())
	{
		CPaintDC dc(this); // 用于绘制的设备上下文

		SendMessage(WM_ICONERASEBKGND, reinterpret_cast<WPARAM>(dc.GetSafeHdc()), 0);

		// 使图标在工作区矩形中居中
		int cxIcon = GetSystemMetrics(SM_CXICON);
		int cyIcon = GetSystemMetrics(SM_CYICON);
		CRect rect;
		GetClientRect(&rect);
		int x = (rect.Width() - cxIcon + 1) / 2;
		int y = (rect.Height() - cyIcon + 1) / 2;

		// 绘制图标
		dc.DrawIcon(x, y, m_hIcon);
	}
	else
	{
		CDialogEx::OnPaint();
	}
}

//当用户拖动最小化窗口时系统调用此函数取得光标
//显示。
HCURSOR CAheadLibExDlg::OnQueryDragIcon()
{
	return static_cast<HCURSOR>(m_hIcon);
}



void CAheadLibExDlg::OnBnClickedButtonInputfile()
{
	// TODO: 在此添加控件通知处理程序代码

	TCHAR wszFilter[] = _T("Dynamic Link Library(*.dll)|*.dll|All Files(*.*)|*.*||");
	CFileDialog dlgFile(TRUE, _T("dll"), NULL, 0, wszFilter, this);
	CString strFilePath;

	if (IDOK == dlgFile.DoModal())
	{
		strFilePath = dlgFile.GetPathName();
		m_editInputFile.SetWindowText(strFilePath);

		m_strFilePath = strFilePath;
		OnLoadFile();
	}
}


void CAheadLibExDlg::OnBnClickedButtonOutputfile()
{
	// TODO: 在此添加控件通知处理程序代码

	TCHAR wszFilter[] = _T("C++ Source(*.cpp)|*.cpp|All Files(*.*)|*.*||");
	CFileDialog dlgFile(FALSE, _T("cpp"), NULL, OFN_HIDEREADONLY | OFN_OVERWRITEPROMPT, wszFilter, this);
	CString strFilePath;

	if (IDOK == dlgFile.DoModal())
	{
		strFilePath = dlgFile.GetPathName();
		m_editOutputFile.SetWindowText(strFilePath);
	}
}


void CAheadLibExDlg::OnDropFiles(HDROP hDropInfo)
{
	// TODO: 在此添加消息处理程序代码和/或调用默认值

	TCHAR szFilePath[MAX_PATH];

	DragQueryFile(hDropInfo, 0, szFilePath, MAX_PATH);
	DragFinish(hDropInfo);
	m_strFilePath = szFilePath;
	OnLoadFile();

	CDialogEx::OnDropFiles(hDropInfo);
}


void CAheadLibExDlg::OnLoadFile()
{
	// TODO: 在此处添加实现代码.

	CString str;

	m_hDll = LoadLibraryEx(m_strFilePath, NULL, LOAD_LIBRARY_AS_IMAGE_RESOURCE);

	m_strFileName = m_strFilePath;
	PathStripPath(m_strFileName.GetBuffer());
	m_strFileName.ReleaseBuffer();

	if (m_hDll == NULL)
	{
		str.Format(_T("Mapping file error! code=%d"), GetLastError());
		AfxMessageBox(str, MB_ICONERROR);
		return;
	}

	PIMAGE_DOS_HEADER pDosHdr = NULL;
	PIMAGE_NT_HEADERS pNtHdr = NULL;
	PIMAGE_NT_HEADERS64 pNtHdr64 = NULL;
	PIMAGE_SECTION_HEADER pSectionHdr = NULL;
	m_bIsx64 = FALSE;
	BOOL bStatus = FALSE;

	CString strExportNameString;
	CString fileArch;
	CString timestamp;
	LPCSTR pszExportNameString;
	CString expEdit;

	//LoadLibraryEx 加载DLL之后，返回指针居然指向4d 5a 之后
	for (int i = 0; i <= 2; i++)
	{
		pDosHdr = (PIMAGE_DOS_HEADER)((ULONG_PTR)m_hDll - i);
		if (pDosHdr->e_magic == IMAGE_DOS_SIGNATURE)
		{
			bStatus = TRUE;
			break;
		}
	}

	if (!bStatus)
	{
		AfxMessageBox(_T("Invalid PE File!"), MB_ICONERROR);
		goto _END;
	}

	if (pDosHdr->e_magic != IMAGE_DOS_SIGNATURE)
	{
		AfxMessageBox(_T("Invalid DOS Header!"), MB_ICONERROR);
		goto _END;
	}

	/*
	* NT Hdr
	*/
	pNtHdr = ImageNtHeader(pDosHdr);
	if (pNtHdr->Signature != IMAGE_NT_SIGNATURE)
	{
		AfxMessageBox(_T("Invalid NT Header!"), MB_ICONERROR);
		goto _END;
	}
	if (pNtHdr->FileHeader.Machine == IMAGE_FILE_MACHINE_AMD64 ||
		pNtHdr->FileHeader.Machine == IMAGE_FILE_MACHINE_IA64)
	{
		m_bIsx64 = TRUE;
		pNtHdr64 = (PIMAGE_NT_HEADERS64)pNtHdr;
	}

	if (!(pNtHdr->FileHeader.Characteristics & IMAGE_FILE_DLL))
	{
		AfxMessageBox(_T("The target is not a dynamic link library!"), MB_ICONERROR);
		goto _END;
	}

	if (m_bIsx64)
	{
		if (pNtHdr64->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].VirtualAddress == 0 ||
			pNtHdr64->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].Size == 0)
		{
			AfxMessageBox(_T("Export table does not exist!"), MB_ICONERROR);
			goto _END;
		}
	}
	else
	{
		if (pNtHdr->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].VirtualAddress == 0 ||
			pNtHdr->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].Size == 0)
		{
			AfxMessageBox(_T("Export table does not exist!"), MB_ICONERROR);
			goto _END;
		}
	}

	pSectionHdr = m_bIsx64 ? IMAGE_FIRST_SECTION(pNtHdr64) : IMAGE_FIRST_SECTION(pNtHdr);

	/*
	* 获取Section
	*/
	m_vecSectionHdrs.clear();

	if (m_bIsx64)
	{
		for (WORD i = 0; i < pNtHdr64->FileHeader.NumberOfSections; i++)
		{
			m_vecSectionHdrs.push_back(*pSectionHdr);
			pSectionHdr++;
		}
	}
	else
	{
		for (WORD i = 0; i < pNtHdr->FileHeader.NumberOfSections; i++)
		{
			m_vecSectionHdrs.push_back(*pSectionHdr);
			pSectionHdr++;
		}
	}
	/*
	* 获取 Export Table
	*/
	PIMAGE_EXPORT_DIRECTORY pExports;
	if (m_bIsx64)
	{
		pExports = (PIMAGE_EXPORT_DIRECTORY)\
			((ULONG_PTR)pDosHdr + pNtHdr64->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].VirtualAddress);
	}
	else
	{
		pExports = (PIMAGE_EXPORT_DIRECTORY)\
			((ULONG_PTR)pDosHdr + pNtHdr->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].VirtualAddress);
	}

	pszExportNameString = (LPCSTR)((ULONG_PTR)pDosHdr + pExports->Name);
	if (IsBadReadPtr(pszExportNameString, sizeof(PUCHAR)) == 0)
	{
		strExportNameString = (WCHAR*)CA2W(pszExportNameString);
	}
	else
	{
		strExportNameString = _T("ERROR!");
	}

	m_vecExportFunc.clear();
	DWORD* pFunc = (DWORD*)(pExports->AddressOfFunctions + (ULONG_PTR)pDosHdr);
	DWORD* nameRVA = (DWORD*)(pExports->AddressOfNames + (ULONG_PTR)pDosHdr);
	int name = 0;

	EXPORT_FUNCTION* exFunc = new EXPORT_FUNCTION;

	for (DWORD Index = 0; Index < pExports->NumberOfFunctions; Index++)
	{
		//
		//默认以序号导出
		//

		exFunc->isOrd = TRUE;
		exFunc->Ordinal = pExports->Base + Index;
		exFunc->FunctionRVA = pFunc[Index];
		exFunc->NameOrdinal = 0;
		exFunc->NameRVA = 0;
		exFunc->Name = _T("N/A");
		ZeroMemory(&exFunc->secInfo, sizeof(IMAGE_SECTION_HEADER));
		exFunc->isUnkown = FALSE;
		exFunc->isFunc = FALSE;
		exFunc->isTranFunc = FALSE;
		exFunc->isData = FALSE;
		exFunc->isDataCount = 0;

		//
		//过滤无效的RVA
		//

		if (exFunc->FunctionRVA == 0)
		{
			continue;
		}

		WORD* ordName = (WORD*)(pExports->AddressOfNameOrdinals + (ULONG_PTR)pDosHdr);
		for (DWORD i = 0; i < pExports->NumberOfNames; i++)
		{
			//
			//查找是否是以名称导出
			//
			if (LOWORD(Index) == *ordName)
			{
				exFunc->isOrd = FALSE;
				exFunc->NameOrdinal = *ordName;
				exFunc->NameRVA = nameRVA[i];
				exFunc->Name = (WCHAR*)CA2W((LPCSTR)((ULONG_PTR)pDosHdr + exFunc->NameRVA));
				name++;

				break;
			}
			ordName++;
		}

		//
		//查找所在区段,定位导出表函数是否是 函数 或 数据 或 中转导出表
		//

		exFunc->isUnkown = TRUE;
		strcpy_s((char*)exFunc->secInfo.Name, strlen("ERROR!"), "ERROR!");
		for (auto& sec : m_vecSectionHdrs)
		{
			if (exFunc->FunctionRVA >= sec.VirtualAddress &&
				exFunc->FunctionRVA <= (sec.VirtualAddress + sec.Misc.VirtualSize))
			{
				memcpy(&exFunc->secInfo, &sec, sizeof(IMAGE_SECTION_HEADER));

				// 				if ((sec.Characteristics & IMAGE_SCN_MEM_EXECUTE) &&
				// 					!(sec.Characteristics & IMAGE_SCN_MEM_WRITE))
				// 				{
				if (sec.Characteristics & IMAGE_SCN_MEM_EXECUTE)
				{
					//
					//可运行不可写 代码区段
					//
					exFunc->isFunc = TRUE;
					exFunc->isUnkown = FALSE;
					break;
				}
				if ((sec.Characteristics & IMAGE_SCN_MEM_READ) &&
					!(sec.Characteristics & IMAGE_SCN_MEM_WRITE))
				{
					//
					//可读不可写 .rdata 区段,一般都是中转导出表
					//

					char* nameTran = (char*)((ULONG_PTR)pDosHdr + exFunc->FunctionRVA);
					if (IsBadReadPtr(nameTran, sizeof(void*)) == 0)
					{
						if (strstr(nameTran, ".") != NULL)
						{
							exFunc->isTranFunc = TRUE;
							exFunc->isUnkown = FALSE;
							exFunc->TranName = (WCHAR*)CA2W((LPCSTR)nameTran);
						}
						else
						{
							//
							//无法识别的函数，不知道怎么处理，只有退出
							//
							str.Format(_T(
								"Unknown .rdata section data! continue?\r\n"
								"ord:%d\r\n"
								"func_rva:%08X\r\n"
								"name:%s"),
								exFunc->Ordinal, exFunc->FunctionRVA, exFunc->Name.GetString());

							AfxMessageBox(str, MB_ICONERROR);
							ExitProcess(-1);

						}
					}
					else
					{
						str.Format(_T(
							"Try to read .rdata section data exception! continue?\r\n"
							"ord:%d\r\n"
							"func_rva:%08X\r\n"
							"name:%s"),
							exFunc->Ordinal, exFunc->FunctionRVA, exFunc->Name.GetString());

						AfxMessageBox(str, MB_ICONERROR);
						ExitProcess(-1);

					}

					break;
				}
				if ((sec.Characteristics & IMAGE_SCN_MEM_READ) &&
					(sec.Characteristics & IMAGE_SCN_MEM_WRITE) &&
					!(sec.Characteristics & IMAGE_SCN_MEM_EXECUTE))
				{
					//
					//可读可写不可运行，数据区段
					//
					exFunc->isData = TRUE;
					exFunc->isUnkown = FALSE;

					//
					//探测数据区段的大小
					//

					if (m_bIsx64)
					{
						uint64_t* probePtr = (uint64_t*)((ULONG_PTR)pDosHdr + exFunc->FunctionRVA);
						if (IsBadReadPtr(probePtr, sizeof(void*)) == 0)
						{
							while (TRUE)
							{
								if (*probePtr != NULL)
								{
									exFunc->isDataCount++;
									probePtr++;
								}
								else
								{
									break;
								}
							}
						}
						else
						{
							str.Format(_T(
								"Try to read .data section data exception!\r\n"
								"ord:%d\r\n"
								"func_rva:%08X\r\n"
								"name:%s"),
								exFunc->Ordinal, exFunc->FunctionRVA, exFunc->Name.GetString());

							AfxMessageBox(str, MB_ICONERROR);
							ExitProcess(-1);
						}
					}
					else
					{
						uint32_t* probePtr = (uint32_t*)((ULONG_PTR)pDosHdr + exFunc->FunctionRVA);
						if (IsBadReadPtr(probePtr, sizeof(void*)) == 0)
						{
							while (TRUE)
							{
								if (*probePtr != NULL)
								{
									exFunc->isDataCount++;
									probePtr++;
								}
								else
								{
									break;
								}
							}
						}
						else
						{
							str.Format(_T(
								"Try to read .data section data exception!\r\n"
								"ord:%d\r\n"
								"func_rva:%08X\r\n"
								"name:%s"),
								exFunc->Ordinal, exFunc->FunctionRVA, exFunc->Name.GetString());

							AfxMessageBox(str, MB_ICONERROR);
							ExitProcess(-1);
						}
					}

					//
					//如果这个导出数据全为空的话，默认给他导出一个指针大小
					//
					if (exFunc->isDataCount == 0)
					{
						exFunc->isDataCount++;
					}

					break;
				}

				AfxMessageBox(_T("Unrecognized export function!"));
				ExitProcess(-1);

				break;
			}
		}

		m_vecExportFunc.push_back(*exFunc);
	}

	delete exFunc;
_END:
	FreeLibrary(m_hDll);
}


void CAheadLibExDlg::OnBnClickedRadioCpp()
{
	// TODO: 在此添加控件通知处理程序代码
	m_editOutputProject.EnableWindow(false);
	GetDlgItem(IDC_BUTTON_OUTPUTPROJECT)->EnableWindow(false);

	m_editOutputFile.EnableWindow(true);
	GetDlgItem(IDC_BUTTON_OUTPUTFILE)->EnableWindow(true);
}


void CAheadLibExDlg::OnBnClickedButtonOutputProject()
{
	// TODO: 在此添加控件通知处理程序代码
	TCHAR wszDir[MAX_PATH] = {};
	BROWSEINFO bi;
	ITEMIDLIST* pItemIdList;

	bi.hwndOwner = m_hWnd;
	bi.iImage = 0;
	bi.lParam = 0;
	bi.lpfn = nullptr;
	bi.lpszTitle = _T("Please Select Your Floder");
	bi.pidlRoot = nullptr;
	bi.pszDisplayName = wszDir;
	bi.ulFlags = BIF_RETURNONLYFSDIRS;

	pItemIdList = SHBrowseForFolder(&bi);

	if (nullptr == pItemIdList)
	{
		AfxMessageBox(_T("Create Floder Browser Error"), MB_ICONERROR);
		return;
	}

	if (SHGetPathFromIDList(pItemIdList, wszDir))
	{
		CString str;
		str.Format(_T("%s\\%s\\"), wszDir, m_strFileName);
		m_editOutputProject.SetWindowText(str);

		m_strProjectPath = str;
	}

}


void CAheadLibExDlg::OnBnClickedRadioProject()
{
	// TODO: 在此添加控件通知处理程序代码

	m_editOutputFile.EnableWindow(false);
	GetDlgItem(IDC_BUTTON_OUTPUTFILE)->EnableWindow(false);

	m_editOutputProject.EnableWindow(true);
	GetDlgItem(IDC_BUTTON_OUTPUTPROJECT)->EnableWindow(true);
}



