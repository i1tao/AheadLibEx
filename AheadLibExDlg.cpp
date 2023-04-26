
// AheadLibExDlg.cpp: 实现文件
//

#include "pch.h"
#include "framework.h"
#include "AheadLibEx.h"
#include "AheadLibExDlg.h"
#include "afxdialogex.h"

#include "AheadLibCommon.h"
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
    , m_bIsx64(false)
    , m_hDll(nullptr)
    , m_bIsWow64(false)

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
    ON_BN_CLICKED(IDOK, &CAheadLibExDlg::OnBnClickedOk)
END_MESSAGE_MAP()


/*
 *	安全取得系统真实信息
 */
VOID SafeGetNativeSystemInfo(__out LPSYSTEM_INFO lpSystemInfo)
{
    if (NULL == lpSystemInfo)    return;
    typedef VOID(WINAPI* LPFN_GetNativeSystemInfo)(LPSYSTEM_INFO lpSystemInfo);
    LPFN_GetNativeSystemInfo fnGetNativeSystemInfo = \
        (LPFN_GetNativeSystemInfo)GetProcAddress(GetModuleHandleA("kernel32"), "GetNativeSystemInfo");

    if (NULL != fnGetNativeSystemInfo)
    {
        fnGetNativeSystemInfo(lpSystemInfo);
    }
    else
    {
        GetSystemInfo(lpSystemInfo);
    }
}

/**
 * 获取系统位数
 */
BOOL IsArch64()
{
    SYSTEM_INFO si;
    SafeGetNativeSystemInfo(&si);
    if (si.wProcessorArchitecture == PROCESSOR_ARCHITECTURE_AMD64 ||
        si.wProcessorArchitecture == PROCESSOR_ARCHITECTURE_IA64)
    {
        return TRUE;
    }

    return FALSE;
}

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

    //检测是否是x64位系统。

    m_bIsWow64 = IsArch64();

    //初始化标题
    SetWindowText(STR_PROC_TITLE);

    //初始化Radio控件
    ((CButton*)GetDlgItem(IDC_RADIO_CPP))->SetCheck(true);
    m_editOutputProject.EnableWindow(false);
    GetDlgItem(IDC_BUTTON_OUTPUTPROJECT)->EnableWindow(false);

    //初始化欢迎信息

    CString strWelcome;
    strWelcome += _T("------------------------------------------------------------------------\r\n");
    strWelcome += _T("Author: " TEXT(" ") STR_AUTHOR_NAME "\r\n");
    strWelcome += (STR_GITHUB_ADDRESS _T("\r\n"));
    strWelcome += STR_COPYRIGHT;
    strWelcome += _T("\r\n");
    strWelcome += _T("------------------------------------------------------------------------\r\n");
    strWelcome += _T("Based on xjun's AheadLib-x86-x64(https://github.com/strivexjun/AheadLib-x86-x64) \r\n");
    strWelcome += _T("------------------------------------------------------------------------\r\n");
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
    CString str;
    str = m_strFileName;
    PathRemoveExtension(str.GetBuffer());
    CFileDialog dlgFile(FALSE, _T("cpp"), m_strFileName.GetLength() ? str : _T("Mydll"), OFN_HIDEREADONLY | OFN_OVERWRITEPROMPT, wszFilter, this);
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
    if (m_bIsWow64)
    {
        PVOID OldValue = NULL;

        //  Disable redirection immediately prior to the native API
        //  function call.
        if (Wow64DisableWow64FsRedirection(&OldValue))
        {
            //  Any function calls in this block of code should be as concise
            //  and as simple as possible to avoid unintended results.
            m_hDll = LoadLibraryEx(m_strFilePath, NULL, LOAD_LIBRARY_AS_IMAGE_RESOURCE);

            //  Immediately re-enable redirection. Note that any resources
            //  associated with OldValue are cleaned up by this call.
            if (FALSE == Wow64RevertWow64FsRedirection(OldValue))
            {
                //  Failure to re-enable redirection should be considered
                //  a critical failure and execution aborted.
                return;
            }
        }

    }
    else
    {
        m_hDll = LoadLibraryEx(m_strFilePath, NULL, LOAD_LIBRARY_AS_IMAGE_RESOURCE);
    }


    m_strFileName = m_strFilePath;
    PathStripPath(m_strFileName.GetBuffer());
    m_strFileName.ReleaseBuffer();

    if (m_hDll == NULL)
    {
        CString strError;
        strError.Format(_T("Mapping file error! code=%d"), GetLastError());
        AfxMessageBox(strError, MB_ICONERROR);
        return;
    }

    OnAnalyzeFile();
}

void CAheadLibExDlg::OnAnalyzeFile()
{
    PIMAGE_DOS_HEADER pDosHdr = NULL;
    PIMAGE_NT_HEADERS pNtHdr = NULL;
    PIMAGE_NT_HEADERS64 pNtHdr64 = NULL;
    PIMAGE_SECTION_HEADER pSectionHdr = NULL;
    CString strError;
    CString strExportNameString;
    CString strFileArch;
    CString strTimeStamp;
    LPCSTR pszExportNameString;
    CString expEdit;
    CString strFileInfo;


    //LoadLibraryEx加载DLL之后，返回指针并不指向模块起始地址

    m_hDll = reinterpret_cast<HMODULE>(reinterpret_cast<ULONG_PTR>(m_hDll) & 0xfffffff0);
    pDosHdr = reinterpret_cast<PIMAGE_DOS_HEADER>(m_hDll);
    if (pDosHdr->e_magic != IMAGE_DOS_SIGNATURE)
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

    if (pNtHdr->FileHeader.Machine == IMAGE_FILE_MACHINE_IA64 ||
        pNtHdr->FileHeader.Machine == IMAGE_FILE_MACHINE_AMD64)
    {
        m_bIsx64 = TRUE;
        pNtHdr64 = reinterpret_cast<PIMAGE_NT_HEADERS64>(pNtHdr);
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

    auto nNumberOfSections = m_bIsx64 ? pNtHdr64->FileHeader.NumberOfSections : pNtHdr->FileHeader.NumberOfSections;

    for (WORD i = 0; i < nNumberOfSections; i++)
    {
        m_vecSectionHdrs.push_back(*pSectionHdr);
        pSectionHdr++;
    }

    /*
    * 获取 Export Table
    */

    PIMAGE_EXPORT_DIRECTORY pExports;
    if (m_bIsx64)
    {
        pExports = reinterpret_cast<PIMAGE_EXPORT_DIRECTORY>(
            reinterpret_cast<ULONG_PTR>(pDosHdr) +
            pNtHdr64->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].VirtualAddress);
    }
    else
    {
        pExports = reinterpret_cast<PIMAGE_EXPORT_DIRECTORY>(
            reinterpret_cast<ULONG_PTR>(pDosHdr) +
            pNtHdr->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].VirtualAddress);
    }

    pszExportNameString = reinterpret_cast<LPCSTR>(reinterpret_cast<ULONG_PTR>(pDosHdr) + pExports->Name);
    if (IsBadReadPtr(pszExportNameString, sizeof(PUCHAR)) == 0)
    {
        strExportNameString = (WCHAR*)CA2W(pszExportNameString);
    }
    else
    {
        strExportNameString = _T("ERROR!");
    }

    m_vecExportFunc.clear();

    DWORD* pFunc = reinterpret_cast<DWORD*>(pExports->AddressOfFunctions + reinterpret_cast<ULONG_PTR>(pDosHdr));
    DWORD* nameRVA = reinterpret_cast<DWORD*>(pExports->AddressOfNames + reinterpret_cast<ULONG_PTR>(pDosHdr));
    int name = 0;

    EXPORT_FUNCTION* exFunc = new EXPORT_FUNCTION;

    for (DWORD Index = 0; Index < pExports->NumberOfFunctions; Index++)
    {
        /*
        * 初始化结构体 默认以序号导出
        */

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

        /*
        * 过滤无效的RVA
        */

        if (exFunc->FunctionRVA == 0)
        {
            continue;
        }

        WORD* ordName = reinterpret_cast<WORD*>(pExports->AddressOfNameOrdinals + reinterpret_cast<ULONG_PTR>(pDosHdr));
        for (DWORD i = 0; i < pExports->NumberOfNames; i++)
        {
            /*
            * 遍历并保存以名称导出的函数
            */

            if (LOWORD(Index) == *ordName)
            {
                exFunc->isOrd = FALSE;
                exFunc->NameOrdinal = *ordName;
                exFunc->NameRVA = nameRVA[i];
                exFunc->Name = (WCHAR*)CA2W(reinterpret_cast<LPCSTR>(reinterpret_cast<ULONG_PTR>(pDosHdr) + exFunc->NameRVA));
                name++;

                break;
            }
            ordName++;
        }

        /*
        * 定位导出表函数是否是 函数 或 数据 或 中转导出表
        */

        exFunc->isUnkown = TRUE;
        strcpy_s((char*)exFunc->secInfo.Name, strlen("ERROR!") + 1, "ERROR!");
        for (auto& sec : m_vecSectionHdrs)
        {
            if (exFunc->FunctionRVA >= sec.VirtualAddress &&
                exFunc->FunctionRVA <= (sec.VirtualAddress + sec.Misc.VirtualSize))
            {
                memcpy(&exFunc->secInfo, &sec, sizeof(IMAGE_SECTION_HEADER));

                if (sec.Characteristics & IMAGE_SCN_MEM_EXECUTE)
                {
                    /*
                    * 可运行不可写 代码区段
                    */
                    exFunc->isFunc = TRUE;
                    exFunc->isUnkown = FALSE;
                    break;
                }
                if ((sec.Characteristics & IMAGE_SCN_MEM_READ) &&
                    !(sec.Characteristics & IMAGE_SCN_MEM_WRITE))
                {
                    /*
                    * 可读不可写 .rdata 区段,一般都是中转导出表
                    */

                    char* nameTran = reinterpret_cast<char*>(reinterpret_cast<ULONG_PTR>(pDosHdr) + exFunc->FunctionRVA);
                    if (IsBadReadPtr(nameTran, sizeof(void*)) == 0)
                    {
                        if (strstr(nameTran, ".") != NULL)
                        {
                            exFunc->isTranFunc = TRUE;
                            exFunc->isUnkown = FALSE;
                            exFunc->TranName = (WCHAR*)CA2W(reinterpret_cast<LPCSTR>(nameTran));
                        }
                        else
                        {
                            /*
                            * 无法识别的函数
                            */
                            strError.Format(_T(
                                "Unknown .rdata section data! continue?\r\n"
                                "ord:%d\r\n"
                                "func_rva:%08X\r\n"
                                "name:%s"),
                                exFunc->Ordinal, exFunc->FunctionRVA, exFunc->Name.GetString());

                            AfxMessageBox(strError, MB_ICONERROR);
                            ExitProcess(-1);
                        }
                    }
                    else
                    {
                        strError.Format(_T(
                            "Try to read .rdata section data exception! continue?\r\n"
                            "ord:%d\r\n"
                            "func_rva:%08X\r\n"
                            "name:%s"),
                            exFunc->Ordinal, exFunc->FunctionRVA, exFunc->Name.GetString());

                        AfxMessageBox(strError, MB_ICONERROR);
                        ExitProcess(-1);
                    }

                    break;
                }
                if ((sec.Characteristics & IMAGE_SCN_MEM_READ) &&
                    (sec.Characteristics & IMAGE_SCN_MEM_WRITE) &&
                    !(sec.Characteristics & IMAGE_SCN_MEM_EXECUTE))
                {
                    /*
                    * 可读可写不可运行，数据区段
                    */
                    exFunc->isData = TRUE;
                    exFunc->isUnkown = FALSE;

                    /*
                    * 探测数据区段的大小
                    */

                    if (m_bIsx64)
                    {
                        uint64_t* probePtr = reinterpret_cast<uint64_t*>(reinterpret_cast<ULONG_PTR>(pDosHdr) + exFunc->FunctionRVA);
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
                            strError.Format(_T(
                                "Try to read .data section data exception!\r\n"
                                "ord:%d\r\n"
                                "func_rva:%08X\r\n"
                                "name:%s"),
                                exFunc->Ordinal, exFunc->FunctionRVA, exFunc->Name.GetString());

                            AfxMessageBox(strError, MB_ICONERROR);
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
                            strError.Format(_T(
                                "Try to read .data section data exception!\r\n"
                                "ord:%d\r\n"
                                "func_rva:%08X\r\n"
                                "name:%s"),
                                exFunc->Ordinal, exFunc->FunctionRVA, exFunc->Name.GetString());

                            AfxMessageBox(strError, MB_ICONERROR);
                            ExitProcess(-1);
                        }
                    }

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


    /*
    * 显示文件信息到对话框
    */

    switch (pNtHdr->FileHeader.Machine)
    {
    case IMAGE_FILE_MACHINE_I386:
        strFileArch = _T("IMAGE_FILE_MACHINE_I386");
        break;
    case IMAGE_FILE_MACHINE_AMD64:
        strFileArch = _T("IMAGE_FILE_MACHINE_AMD64");
        break;
    case IMAGE_FILE_MACHINE_IA64:
        strFileArch = _T("IMAGE_FILE_MACHINE_IA64");
        break;
    default:
        strFileArch.Format(_T("Machine ID Unknown id->%d"), pNtHdr->FileHeader.Machine);
        break;
    }

    tm t;
    errno_t er = localtime_s(&t, (const time_t*)&pNtHdr->FileHeader.TimeDateStamp);
    if (er == 0)
    {
        wchar_t wszBuffer[32] = {};
        er = _tasctime_s(wszBuffer, 32, &t);
        strTimeStamp = wszBuffer;
    }

    for (auto element : m_vecExportFunc)
    {
        if (element.isFunc)
        {
            strError.Format(_T("%04X    %08X    %s | %hs\r\n"),
                element.Ordinal, element.FunctionRVA, element.Name.GetString(), element.secInfo.Name);
        }
        else if (element.isTranFunc)
        {
            strError.Format(_T("%04X    %08X    %s | %hs | %s\r\n"),
                element.Ordinal, element.FunctionRVA, element.Name.GetString(), element.secInfo.Name, element.TranName.GetString());
        }
        else if (element.isData)
        {
            strError.Format(_T("%04X    %08X    %s | %hs | DATA<%d>\r\n"),
                element.Ordinal, element.FunctionRVA, element.Name.GetString(), element.secInfo.Name, element.isDataCount);
        }
        else if (element.isUnkown)
        {
            strError.Format(_T("%04X    %08X    %s | %hs | ???\r\n"),
                element.Ordinal, element.FunctionRVA, element.Name.GetString(), element.secInfo.Name);
        }
        else
        {
            AfxMessageBox(_T("???????"));
            ExitProcess(-2);
        }

        expEdit += strError;
    }

    strFileInfo += "NameString: ";
    strFileInfo += m_strFileName;
    strFileInfo += "\r\n";
    strFileInfo += "Architecture: ";
    strFileInfo += strFileArch;
    strFileInfo += "\r\n";
    strFileInfo += "TimeStamp: ";
    strFileInfo += strTimeStamp;
    strFileInfo += "\r\n\r\n";
    strFileInfo += expEdit;

    m_editInfo.SetWindowText(strFileInfo);
_END:
    FreeLibrary(m_hDll);
}

void CAheadLibExDlg::OnCreateCppSource(CString& strSource, CString& strAsmSource)
{
    CString str;
    strSource += g_szCppHeader;

    for (auto ExportFunc : m_vecExportFunc)
    {
        if (ExportFunc.isTranFunc) //
        {
            str.Format(_T("#pragma comment(linker, \"/EXPORT:%s=%s,@%d\")\r\n"),
                ExportFunc.Name.GetString(), ExportFunc.TranName.GetString(), ExportFunc.Ordinal);
        }
        else if (ExportFunc.isOrd) //序号
        {
            if (m_bIsx64)
            {
                str.Format(_T("#pragma comment(linker, \"/EXPORT:Noname%d=AheadLibEx_Unnamed%d,@%d,NONAME\")\r\n"),
                    ExportFunc.Ordinal, ExportFunc.Ordinal, ExportFunc.Ordinal);
            }
            else
            {
                str.Format(_T("#pragma comment(linker, \"/EXPORT:Noname%d=_AheadLibEx_Unnamed%d,@%d,NONAME\")\r\n"),
                    ExportFunc.Ordinal, ExportFunc.Ordinal, ExportFunc.Ordinal);
            }

        }
        else //名称
        {
            if (m_bIsx64)
            {
                str.Format(_T("#pragma comment(linker, \"/EXPORT:%s=AheadLibEx_%s,@%d\")\r\n"),
                    ExportFunc.Name.GetString(), ExportFunc.Name.GetString(), ExportFunc.Ordinal);
            }
            else
            {
                str.Format(_T("#pragma comment(linker, \"/EXPORT:%s=_AheadLibEx_%s,@%d\")\r\n"),
                    ExportFunc.Name.GetString(), ExportFunc.Name.GetString(), ExportFunc.Ordinal);
            }

        }
        strSource += str;
    }

    strSource += _T("\r\n");

    //全局导出变量定义
    for (auto ExportFunc : m_vecExportFunc)
    {
        if (ExportFunc.isTranFunc)
        {
            continue;
        }

        if (ExportFunc.isData)
        {
            if (ExportFunc.isOrd)
            {
                str.Format(_T("EXTERN_C PVOID AheadLibEx_Unnamed%d[%d] = { 0 };\r\n"),
                    ExportFunc.Ordinal, ExportFunc.isDataCount);
            }
            else
            {
                str.Format(_T("EXTERN_C PVOID AheadLibEx_%s[%d] = { 0 };\r\n"),
                    ExportFunc.Name.GetString(), ExportFunc.isDataCount);
            }
            strSource += str;
        }

    }

    strSource += _T("\r\n");


    if (m_bIsx64)
    {
        strSource += _T("extern \"C\" \n{\r\n");
    }

    for (auto ExportFunc : m_vecExportFunc)
    {
        if (ExportFunc.isTranFunc)
        {
            continue;
        }

        if (ExportFunc.isOrd)
        {
            str.Format(_T("PVOID pfnAheadLibEx_Unnamed%d;\r\n"),
                ExportFunc.Ordinal);
        }
        else
        {
            str.Format(_T("PVOID pfnAheadLibEx_%s;\r\n"),
                ExportFunc.Name.GetString());
        }

        strSource += str;

    }

    if (m_bIsx64)
    {
        strSource += _T("}\r\n");
    }

    strSource += _T("\r\n");

    /*
    * 其他代码
    */
    CString g_init;

    strSource += g_Free;

    str = g_Load;
    str.Replace(_T("AHEADLIB_XXXXXX.dll"), m_strFileName.GetString());
    strSource += str;

    strSource += g_GetAddress;

    //生成Init函数代码
    g_init = _T("BOOL WINAPI Init()\r\n{\r\n");

    for (auto ExportFunc : m_vecExportFunc)
    {
        if (ExportFunc.isTranFunc)
        {
            continue;
        }

        if (ExportFunc.isOrd)
        {
            str.Format(_T("\tpfnAheadLibEx_Unnamed%d = GetAddress(MAKEINTRESOURCEA(%d));\r\n"),
                ExportFunc.Ordinal, ExportFunc.Ordinal);
        }
        else
        {
            str.Format(_T("\tpfnAheadLibEx_%s = GetAddress(\"%s\");\r\n"),
                ExportFunc.Name.GetString(), ExportFunc.Name.GetString());
        }

        g_init += str;

        if (ExportFunc.isData)
        {
            if (ExportFunc.isOrd)
            {
                str.Format(_T("\tmemcpy(AheadLibEx_Unnamed%d,pfnAheadLibEx_Unnamed%d,sizeof(PVOID) * %d);\r\n"),
                    ExportFunc.Ordinal, ExportFunc.Ordinal, ExportFunc.isDataCount);
            }
            else
            {
                str.Format(_T("\tmemcpy(AheadLibEx_%s,pfnAheadLibEx_%s,sizeof(PVOID) * %d);\r\n"),
                    ExportFunc.Name.GetString(), ExportFunc.Name.GetString(), ExportFunc.isDataCount);
            }

            g_init += str;
        }

    }

    g_init += _T("\treturn TRUE;\r\n");
    g_init += _T("}\t\n");

    strSource += g_init;
    strSource += g_ThreadProc;
    strSource += g_Dllmain;


    /*
    * 生成.asm
    */

    if (m_bIsx64)
    {
        strAsmSource += g_szAsmHeader;

        strAsmSource += _T(".DATA\r\n");

        for (auto ExportFun : m_vecExportFunc)
        {
            if (ExportFun.isTranFunc)
            {
                continue;
            }
            if (ExportFun.isData)
            {
                continue;
            }

            if (ExportFun.isOrd)
            {
                str.Format(_T("EXTERN pfnAheadLibEx_Unnamed%d:dq;\r\n"),
                    ExportFun.Ordinal);
            }
            else
            {
                str.Format(_T("EXTERN pfnAheadLibEx_%s:dq;\r\n"),
                    ExportFun.Name.GetString());
            }

            strAsmSource += str;
        }

        strAsmSource += _T("\r\n.CODE\r\n");

        for (auto ExportFun : m_vecExportFunc)
        {
            if (ExportFun.isTranFunc)
            {
                continue;
            }
            if (ExportFun.isData)
            {
                continue;
            }

            if (ExportFun.isOrd)
            {
                str.Format(_T(
                    "AheadLibEx_Unnamed%d PROC\r\n"
                    "\tjmp pfnAheadLibEx_Unnamed%d\r\n"
                    "AheadLibEx_Unnamed%d ENDP\r\n\r\n"),
                    ExportFun.Ordinal, ExportFun.Ordinal, ExportFun.Ordinal);
            }
            else
            {
                str.Format(_T(
                    "AheadLibEx_%s PROC\r\n"
                    "\tjmp pfnAheadLibEx_%s\r\n"
                    "AheadLibEx_%s ENDP\r\n\r\n"),
                    ExportFun.Name.GetString(), ExportFun.Name.GetString(), ExportFun.Name.GetString());
            }

            strAsmSource += str;
        }

        strAsmSource += _T("\r\nEND\r\n");

    }
    else
    {
        for (auto ExportFun : m_vecExportFunc)
        {
            if (ExportFun.isTranFunc)
            {
                continue;
            }
            if (ExportFun.isData)
            {
                continue;
            }

            if (ExportFun.isOrd)
            {
                str.Format(_T("EXTERN_C __declspec(naked) void __cdecl AheadLibEx_Unnamed%d(void)\r\n"
                    "{\r\n"
                    "\t__asm jmp pfnAheadLibEx_Unnamed%d;\r\n"
                    "}\r\n"),
                    ExportFun.Ordinal, ExportFun.Ordinal);
            }
            else
            {
                str.Format(_T("EXTERN_C __declspec(naked) void __cdecl AheadLibEx_%s(void)\r\n"
                    "{\r\n"
                    "\t__asm jmp pfnAheadLibEx_%s;\r\n"
                    "}\r\n"),
                    ExportFun.Name.GetString(), ExportFun.Name.GetString());
            }

            strSource += str;
            strSource += _T("\r\n");

        }
    }
}

void CAheadLibExDlg::OnCreateSln(CString& strSln)
{
    CString str;

    strSln += g_szSlnHeader;

    GUID guidProject;
    GUID guidVcxproj;
    GUID guidSolution;


    HRESULT h = CoCreateGuid(&guidProject);
    if (h != S_OK)
    {
        str.Format(_T("Create Sln Guid Error \r\n"));
        AfxMessageBox(str, MB_ICONERROR);
        return;
    }
    h = CoCreateGuid(&guidVcxproj);
    if (h != S_OK)
    {
        str.Format(_T("Create Vcxproj Guid Error \r\n"));
        AfxMessageBox(str, MB_ICONERROR);
        return;
    }
    h = CoCreateGuid(&guidSolution);
    if (h != S_OK)
    {
        str.Format(_T("Create Solution Guid Error \r\n"));
        AfxMessageBox(str, MB_ICONERROR);
        return;
    }

    m_strGuidProject.Format(_T("{%08X-%04X-%04X-%02X%02X-%02X%02X%02X%02X%02X%02X}"),
        guidProject.Data1,
        guidProject.Data2,
        guidProject.Data3,
        guidProject.Data4[0],
        guidProject.Data4[1],
        guidProject.Data4[2],
        guidProject.Data4[3],
        guidProject.Data4[4],
        guidProject.Data4[5],
        guidProject.Data4[6],
        guidProject.Data4[7]
    );

    m_strGuidVcxproj.Format(_T("{%08X-%04X-%04X-%02X%02X-%02X%02X%02X%02X%02X%02X}"),
        guidVcxproj.Data1,
        guidVcxproj.Data2,
        guidVcxproj.Data3,
        guidVcxproj.Data4[0],
        guidVcxproj.Data4[1],
        guidVcxproj.Data4[2],
        guidVcxproj.Data4[3],
        guidVcxproj.Data4[4],
        guidVcxproj.Data4[5],
        guidVcxproj.Data4[6],
        guidVcxproj.Data4[7]
    );

    m_strGuidSolution.Format(_T("{%08X-%04X-%04X-%02X%02X-%02X%02X%02X%02X%02X%02X}"),
        guidSolution.Data1,
        guidSolution.Data2,
        guidSolution.Data3,
        guidSolution.Data4[0],
        guidSolution.Data4[1],
        guidSolution.Data4[2],
        guidSolution.Data4[3],
        guidSolution.Data4[4],
        guidSolution.Data4[5],
        guidSolution.Data4[6],
        guidSolution.Data4[7]
    );
    //Project("{8BC9CEB8-8B4A-11D0-8D11-00A0C91BC942}") = "CppTest", "CppTest.vcxproj", "{4388BEEF-2A07-4C51-BB41-807B9A3E918E}"

    CString strProjectName;
    strProjectName = m_strFileName;

    PathRemoveExtension(strProjectName.GetBuffer());
    strProjectName.ReleaseBuffer();

    str.Format(_T("Project(\"%s\") = \"%s\", \"%s.vcxproj\", \"%s\""),
        m_strGuidProject.GetString(), strProjectName.GetString(), strProjectName.GetString(), m_strGuidVcxproj.GetString());
    strSln += str;
    strSln += "\r\nEndProject\r\n";

    strSln += L"Global\r\n\
\tGlobalSection(SolutionConfigurationPlatforms) = preSolution\r\n\
\t\tDebug|x64 = Debug|x64\r\n\
\t\tDebug|x86 = Debug|x86\r\n\
\t\tRelease|x64 = Release|x64\r\n\
\t\tRelease|x86 = Release|x86\r\n\
\tEndGlobalSection\r\n\
\tGlobalSection(ProjectConfigurationPlatforms) = postSolution\r\n";
    //{4388BEEF - 2A07 - 4C51 - BB41 - 807B9A3E918E}.Debug | x64.ActiveCfg = Debug | x64
    str.Format(_T("\t\t%s.Debug|x64.ActiveCfg = Debug|x64\r\n"), m_strGuidVcxproj.GetString());
    strSln += str;
    str.Format(_T("\t\t%s.Debug|x64.Build.0 = Debug|x64\r\n"), m_strGuidVcxproj.GetString());
    strSln += str;
    str.Format(_T("\t\t%s.Debug|x86.ActiveCfg = Debug|Win32\r\n"), m_strGuidVcxproj.GetString());
    strSln += str;
    str.Format(_T("\t\t%s.Debug|x86.Build.0 = Debug|Win32\r\n"), m_strGuidVcxproj.GetString());
    strSln += str;
    str.Format(_T("\t\t%s.Release|x64.ActiveCfg = Release|x64\r\n"), m_strGuidVcxproj.GetString());
    strSln += str;
    str.Format(_T("\t\t%s.Release|x64.Build.0 = Release|x64\r\n"), m_strGuidVcxproj.GetString());
    strSln += str;
    str.Format(_T("\t\t%s.Release|x86.ActiveCfg = Release|Win32\r\n"), m_strGuidVcxproj.GetString());
    strSln += str;
    str.Format(_T("\t\t%s.Release|x86.Build.0 = Release|Win32\r\n"), m_strGuidVcxproj.GetString());
    strSln += str;

    strSln += L"\tEndGlobalSection\r\n \
\tGlobalSection(SolutionProperties) = preSolution\r\n \
\t\tHideSolutionNode = FALSE \r\n\
\tEndGlobalSection \r\n\
\tGlobalSection(ExtensibilityGlobals) = postSolution\r\n";

    str.Format(_T("\t\tSolutionGuid = %s\r\n"), m_strGuidSolution.GetString());
    strSln += str;
    strSln += L"\tEndGlobalSection\r\nEndGlobal";

}
void CAheadLibExDlg::OnCreateVcxproj(CString& strVcxproj)
{
    CString str;

    strVcxproj += g_szVcxProjectHeader;

    str.Format(_T("  <ItemGroup>\r\n <ClCompile Include=\"%s.cpp\" />\r\n  </ItemGroup>\r\n"), m_strFileNameNOExtension.GetString());
    strVcxproj += str;

    if (m_bIsx64)
    {
        str.Format(_T("  <ItemGroup>\r\n <MASM Include=\"%s\" />\r\n  </ItemGroup>\r\n"), m_strAsmName.GetString());
        strVcxproj += str;
    }
    str.Format(_T("  <PropertyGroup Label=\"Globals\">\r\n\
    <VCProjectVersion>16.0</VCProjectVersion>\r\n\
    <Keyword>Win32Proj</Keyword>\r\n\
    <ProjectGuid>%s</ProjectGuid>\r\n\
    <RootNamespace>%s</RootNamespace>\r\n\
    <WindowsTargetPlatformVersion>10.0</WindowsTargetPlatformVersion>\r\n\
  </PropertyGroup>"),m_strGuidVcxproj.GetString(),m_strFileNameNOExtension.GetString());
    strVcxproj += g_szVcxProjectEnd;
}
void CAheadLibExDlg::OnCreateFilters(CString& strFilters)
{
    CString str;

    strFilters += LR"(<?xml version="1.0" encoding="utf-8"?>
<Project ToolsVersion="4.0" xmlns="http://schemas.microsoft.com/developer/msbuild/2003">
  <ItemGroup>
    <Filter Include="Source Files">)";

    GUID guidSourceFiles;
    GUID guidHeaderFiles;
    GUID guidResourceFiles;

    CString strGuidSourceFiles;
    CString strGuidHeaderFiles;
    CString strGuidResourceFiles;

    HRESULT h = CoCreateGuid(&guidSourceFiles);
    if (h != S_OK)
    {
        str.Format(_T("Create ResourceFiles Guid Error \r\n"));
        AfxMessageBox(str, MB_ICONERROR);
        return;
    }
    h = CoCreateGuid(&guidHeaderFiles);
    if (h != S_OK)
    {
        str.Format(_T("Create HeaderFiles Guid Error \r\n"));
        AfxMessageBox(str, MB_ICONERROR);
        return;
    }
    h = CoCreateGuid(&guidResourceFiles);
    if (h != S_OK)
    {
        str.Format(_T("Create ResourceFiles Guid Error \r\n"));
        AfxMessageBox(str, MB_ICONERROR);
        return;
    }

    strGuidSourceFiles.Format(_T("{%08X-%04X-%04X-%02X%02X-%02X%02X%02X%02X%02X%02X}"),
        guidSourceFiles.Data1,
        guidSourceFiles.Data2,
        guidSourceFiles.Data3,
        guidSourceFiles.Data4[0],
        guidSourceFiles.Data4[1],
        guidSourceFiles.Data4[2],
        guidSourceFiles.Data4[3],
        guidSourceFiles.Data4[4],
        guidSourceFiles.Data4[5],
        guidSourceFiles.Data4[6],
        guidSourceFiles.Data4[7]
    );

    strGuidHeaderFiles.Format(_T("{%08X-%04X-%04X-%02X%02X-%02X%02X%02X%02X%02X%02X}"),
        guidHeaderFiles.Data1,
        guidHeaderFiles.Data2,
        guidHeaderFiles.Data3,
        guidHeaderFiles.Data4[0],
        guidHeaderFiles.Data4[1],
        guidHeaderFiles.Data4[2],
        guidHeaderFiles.Data4[3],
        guidHeaderFiles.Data4[4],
        guidHeaderFiles.Data4[5],
        guidHeaderFiles.Data4[6],
        guidHeaderFiles.Data4[7]
    );

    strGuidResourceFiles.Format(_T("{%08X-%04X-%04X-%02X%02X-%02X%02X%02X%02X%02X%02X}"),
        guidResourceFiles.Data1,
        guidResourceFiles.Data2,
        guidResourceFiles.Data3,
        guidResourceFiles.Data4[0],
        guidResourceFiles.Data4[1],
        guidResourceFiles.Data4[2],
        guidResourceFiles.Data4[3],
        guidResourceFiles.Data4[4],
        guidResourceFiles.Data4[5],
        guidResourceFiles.Data4[6],
        guidResourceFiles.Data4[7]
    );
    str.Format(_T("      <UniqueIdentifier>%s</UniqueIdentifier>"), strGuidHeaderFiles.GetString());
    strFilters += str;
    strFilters += LR"(
      <Extensions>cpp;c;cc;cxx;c++;cppm;ixx;def;odl;idl;hpj;bat;asm;asmx</Extensions>
    </Filter>
    <Filter Include="Header Files">)";

    str.Format(_T("      <UniqueIdentifier>%s</UniqueIdentifier>"), strGuidHeaderFiles.GetString());
    strFilters += str;
    strFilters += LR"(
      <Extensions>h;hh;hpp;hxx;h++;hm;inl;inc;ipp;xsd</Extensions>
    </Filter>
    <Filter Include="Resource Files">)";


    str.Format(_T("      <UniqueIdentifier>%s</UniqueIdentifier>"), strGuidResourceFiles.GetString());
    strFilters += str;
    strFilters += LR"(
      <Extensions>rc;ico;cur;bmp;dlg;rc2;rct;bin;rgs;gif;jpg;jpeg;jpe;resx;tiff;tif;png;wav;mfcribbon-ms</Extensions>
    </Filter>
  </ItemGroup>
  <ItemGroup>
)";

    str.Format(_T("    <ClCompile Include=\"%s.cpp\">"), m_strFileNameNOExtension.GetString());
    strFilters += str;
    strFilters += LR"(
      <Filter>Source Files</Filter>
    </ClCompile>
  </ItemGroup>
  <ItemGroup>
)";
    str.Format(_T("    <MASM Include=\"%s\">"), m_strAsmName.GetString());
    strFilters += str;

    strFilters += LR"(
      <Filter>Source Files</Filter>
    </MASM>
  </ItemGroup>
</Project>)";

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
    bi.lpszTitle = _T("Please Select Your Project Floder");
    bi.pidlRoot = nullptr;
    bi.pszDisplayName = wszDir;
    bi.ulFlags = BIF_RETURNONLYFSDIRS | BIF_EDITBOX | BIF_NEWDIALOGSTYLE;

    pItemIdList = SHBrowseForFolder(&bi);

    if (nullptr == pItemIdList)
    {
        AfxMessageBox(_T("Please choose a floder"), MB_ICONERROR);
        return;
    }

    if (SHGetPathFromIDList(pItemIdList, wszDir))
    {
        CString str;
        CFileFind ff;
        if (m_strFileName.GetLength())
        {
            CString strFileName;
            strFileName = m_strFileName;
            PathRemoveExtension(strFileName.GetBuffer());

            strFileName.ReleaseBuffer();
            str.Format(_T("%s\\AheadLibEx_%s\\"), wszDir, strFileName.GetString());
        }
        else
        {
            str.Format(_T("%s\\AheadLibEx_Project\\"), wszDir);
        }
        m_editOutputProject.SetWindowText(str);

        m_strProjectPath = str;


        if (!ff.FindFile(str))
        {
            CreateDirectory(str, NULL);
        }
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


void CAheadLibExDlg::OnBnClickedOk()
{
    // TODO: 在此添加控件通知处理程序代码
    //CDialogEx::OnOK();

    /*
    * 不管三七二十一，初始化路径字符串
    */
    m_editInputFile.GetWindowText(m_strFilePath);
    m_editOutputProject.GetWindowText(m_strProjectPath);
    m_strFileName = PathFindFileName(m_strFilePath);
    /*
    * 初始化一些字符串
    */

    m_strFileNameNOExtension = m_strFileName;
    PathRemoveExtension(m_strFileNameNOExtension.GetBuffer());
    m_strFileNameNOExtension.ReleaseBuffer();

    m_strAsmName = m_strFileNameNOExtension;
    m_strAsmName += "_jump.asm";
    /*
    * 生成文件
    */

    CString source;
    CString source_asm;

    OnCreateCppSource(source, source_asm);

    if (((CButton*)GetDlgItem(IDC_RADIO_CPP))->GetCheck())
    {
        CString outputPath;
        CFile fileOut;
        CStringA ansiSource;

        m_editOutputFile.GetWindowText(outputPath);

        if (fileOut.Open(outputPath, CFile::modeCreate | CFile::modeWrite))
        {
            ansiSource = CW2CW(source.GetString());
            fileOut.Write(ansiSource.GetString(), ansiSource.GetLength());
            fileOut.Close();

            AfxMessageBox(_T("Generate code success!"), MB_ICONINFORMATION);
        }

        if (m_bIsx64)
        {
            CFile fileOutAsm;
            CString outputPathAsm;
            CStringA ansiSourceAsm;

            _tcscpy_s(outputPathAsm.GetBuffer(outputPath.GetLength() + 16), outputPath.GetLength() + 16, outputPath.GetString());
            PathRenameExtension(outputPathAsm.GetBuffer(), _T("_jump.asm"));
            outputPathAsm.ReleaseBuffer();

            if (fileOutAsm.Open(outputPathAsm, CFile::modeCreate | CFile::modeWrite))
            {
                ansiSourceAsm = CW2CW(source_asm.GetString());
                fileOutAsm.Write(ansiSourceAsm.GetString(), ansiSourceAsm.GetLength());
                fileOutAsm.Close();
            }
        }

    }
    else
    {
        CString strSln;
        CString strSlnPath;
        CString strVcxproj;
        CString strVcxprojPath;
        CString strFilters;
        CString strFiltersPath;

        CString strSourcePath;
        CString StrAsmPath;

        CStringA ansiSln;
        CStringA ansiVcxproj;
        CStringA ansiFilters;

        CStringA ansiSource;
        CStringA ansiSourceAsm;
        CFile file;
        CString str;
        CFileException fileException;

        strSlnPath = m_strProjectPath;
        strSlnPath += m_strFileNameNOExtension;
        strSlnPath += _T(".sln");

        strVcxprojPath = m_strProjectPath;
        strVcxprojPath += m_strFileNameNOExtension;
        strVcxprojPath += _T(".vcxproj");

        strFiltersPath = m_strProjectPath;
        strFiltersPath += m_strFileNameNOExtension;
        strFiltersPath += _T(".vcxproj.filters");

        strSourcePath = m_strProjectPath;
        strSourcePath += m_strFileNameNOExtension;
        strSourcePath += _T(".cpp");
        StrAsmPath = m_strProjectPath;
        StrAsmPath += m_strFileNameNOExtension;
        StrAsmPath += _T("_jump.asm");

        OnCreateSln(strSln);
        OnCreateVcxproj(strVcxproj);
        OnCreateFilters(strFilters);

        if (file.Open(strSlnPath, CFile::modeCreate | CFile::modeWrite, &fileException))
        {
            ansiSln = CW2CW(strSln.GetString());
            file.Write(ansiSln.GetString(), ansiSln.GetLength());
            file.Close();
        }
        else
        {
            AfxMessageBox(_T("Write Sln error!"), MB_ICONERROR);
            return;
        }

        if (file.Open(strVcxprojPath, CFile::modeCreate | CFile::modeWrite, &fileException))
        {
            ansiVcxproj = CW2CW(strVcxproj.GetString());
            file.Write(ansiVcxproj.GetString(), ansiVcxproj.GetLength());
            file.Close();
        }
        else
        {
            AfxMessageBox(_T("Write Vcxproj error!"), MB_ICONERROR);
            return;
        }

        if (file.Open(strFiltersPath, CFile::modeCreate | CFile::modeWrite, &fileException))
        {
            ansiFilters = CW2CW(strFilters.GetString());
            file.Write(ansiFilters.GetString(), ansiFilters.GetLength());
            file.Close();
        }
        else
        {
            AfxMessageBox(_T("Write Vcxproj.filters error!"), MB_ICONERROR);
            return;
        }


        if (file.Open(strSourcePath, CFile::modeCreate | CFile::modeWrite))
        {
            ansiSource = CW2CW(source.GetString());
            file.Write(ansiSource.GetString(), ansiSource.GetLength());
            file.Close();
        }
        else
        {
            AfxMessageBox(_T("Write .cpp error!"), MB_ICONERROR);
            return;
        }

        if (m_bIsx64)
        {
            if (file.Open(StrAsmPath, CFile::modeCreate | CFile::modeWrite))
            {
                ansiSourceAsm = CW2CW(source_asm.GetString());
                file.Write(ansiSourceAsm.GetString(), ansiSourceAsm.GetLength());
                file.Close();
            }
            else
            {
                AfxMessageBox(_T("Write .asm error!"), MB_ICONERROR);
                return;
            }
        }
        AfxMessageBox(_T("Generate code success!"), MB_ICONINFORMATION);
    }
}
